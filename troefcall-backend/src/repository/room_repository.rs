use rocket::http::Status;
use rocket::http::ext::IntoCollection;
use rocket::tokio::sync::Mutex;

use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::atomic::AtomicUsize;

type RoomId = usize;
type Map<K, V> = Mutex<HashMap<K, V>>;
type Error<'a> = (Status, &'a str);

use crate::controller::password;
use crate::model::game::{Room, Player};
pub struct RoomRepository {
    rooms: Map<RoomId, Room>, //stores rooms by room id
    hosts: Map<UserId, RoomId>, //stores room ids by host user id
    room_count: AtomicUsize, //generates new ids for rooms
}

impl Default for RoomRepository {
    fn default() -> Self {
        RoomRepository {
            rooms: Mutex::new(HashMap::new()),
            room_count: AtomicUsize::new(1),
            hosts: Mutex::new(HashMap::new()),
        }
    }
}
impl RoomRepository{
    pub fn test_repo(user_repo: &mut UserRepository) -> Self {
        println!("Creating room repository with test data.");
        let mut room_amt: usize = 30;
        let users = user_repo.users.get_mut();
        if users.len() - 4 < room_amt {
            let users_amt = users.len();
            println!("Not enough users in user repo to create test {room_amt} rooms! ({users_amt} users in user repo)");
            room_amt = users_amt - 4;
        }
        let password = password::hash_password("asdf").unwrap_or_else(|_| "".to_string());

        const ADMIN_ID:usize = 0;
        const ROOM_ID:usize = 0;    //just for clarity
        let mut admin_room = Room::new(ROOM_ID, "Room 0 with a super long name for some reason, good luck displaying this properly".to_string(),
                password,
                users.get(&ADMIN_ID).unwrap());
        for i in 1..4 {
            //add 3 players to admin room
            let user = users.get_mut(&(users.len() - i )).unwrap();
            user.current_room = Some(ROOM_ID);
            let player = Player::from(&user.clone());
            let _ = admin_room.add_player(player, i);
        }
        let mut rooms_map = HashMap::from([
            (ADMIN_ID, admin_room )
        ]);
        let mut hosts_map = HashMap::from([
            (0, 0),
        ]);
        users.get_mut(&ADMIN_ID).unwrap().current_room = Some(0);
        //create test rooms
        for i in 1..room_amt {
            let room_id = i;
            let room_name = format!("Room {}", i);
            let host_user = users.get(&i).unwrap();
            println!("Creating room {} with host {}", room_name, host_user.username);
            let room = Room::new(room_id, room_name, "".to_string(), host_user);
            rooms_map.insert(room_id, room);
            hosts_map.insert(i, room_id);
            users.get_mut(&i).unwrap().current_room = Some(room_id);
        }
        RoomRepository {
            rooms: Mutex::new(rooms_map),
            room_count: AtomicUsize::new(100),
            hosts: Mutex::new(hosts_map),
        }
    }
}

use crate::model::login::{UserId, User, Role};
use super::UserRepository;

impl RoomRepository {
    pub async fn get_rooms_count(&self) -> usize {
        self.rooms.lock().await.len()
    }
    
    /// Returns a vector of all rooms in the repository, sorted by id. Uses pagination.
    pub async fn get_rooms_paged(&self, start: usize, count: usize, only_public: bool) -> (usize, Vec<Room>) {
        let mut rooms = self.rooms.lock().await.values()
            .filter( |room| room.game_in_progress == false
                & (only_public == false || room.password.len() == 0) )
            .cloned()
            .collect::<Vec<Room>>();
        rooms.sort_by_key(|room| room.id);
        let page_amt = (rooms.len()+count-1) / count;
        (page_amt, rooms.iter().skip(start).take(count).cloned().collect())
    }

    // pub async fn get_room_by_host(&self, host_user_id: UserId) -> Result<Room, &'static str> {
    //     let hosts = self.hosts.lock().await;
    //     let room_id = hosts.get(&host_user_id).cloned()
    //         .ok_or("User is not hosting a room!")?;
    //     let rooms = self.rooms.lock().await;
    //     rooms.get(&room_id).cloned()
    //         .ok_or("Room no longer exists!")
    // }
    
    /// Returns whether the user with the given `user_id` is hosting the room with the given `room_id`.
    pub async fn user_is_host<'a> (&self, user_id: usize, room_id: RoomId) -> bool {
        let hosts = self.hosts.lock().await;
        hosts.get(&user_id) == Some(&room_id)
    }

    pub async fn get_room_by_id<'a> (&self, room_id: RoomId) -> Result<Room, Error> {
        let rooms = self.rooms.lock().await;
        rooms.get(&room_id).cloned().ok_or((Status::Gone, "Room not found!"))
    }

    pub async fn create_room<'a> (&self, host_user: &mut User, name: String, password: String) -> Result<Room, Error<'a>> {
        if host_user.current_room.is_some() {
            return Err((Status::Conflict, "User is already in a room! Leave the room before creating a new one!"));
        }
        if name.len() < 3 { // check if name is valid
            return Err((Status::BadRequest, "Room name must be at least 3 characters long!"));
        }
        let mut hosts = self.hosts.lock().await;
        if hosts.contains_key(&host_user.id) { // check if user is already hosting a room
            return Err((Status::BadRequest, "User is already hosting a room!"));
        }
        // create room
        let room_id = self.room_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let pwd_hash = {
            if password.len() > 0 { //only hash password if it's not empty; need to be able to see if room is public clearly
                password::hash_password(password.as_str())?
            }
            else { //if password is empty, don't hash it; front end will be able to check if password is empty
                "".to_string()
            }
        };
        let new_room = Room::new(room_id, name.to_string(), pwd_hash, host_user);
        
        // add room to repository
        let mut rooms = self.rooms.lock().await;
        hosts.insert(host_user.id, room_id);
        rooms.insert(room_id, new_room.clone());
        host_user.current_room = Some(new_room.id.clone());
        Ok(new_room)
    }

    pub async fn update_room(&self, room: Room) -> bool {
        let mut rooms = self.rooms.lock().await;
        if rooms.contains_key(&room.id) {
            rooms.insert(room.id, room.clone());
            true
        } else {
            false
        }
    }

    /// This function transfers the host of a room from one user to another.
    /// The user with the given `from_user_id` is removed as a host of the room,
    /// and the user with the given `to_user_id` is added as a host of the room.
    /// Returns `true` if the transfer was successful, or `false` if the
    /// transfer failed because the user with the given `from_user_id` was not
    /// a host of the room. 
    pub async fn transfer_host(&self, from_user_id: UserId, to_user_id: UserId) -> bool {
        let mut hosts = self.hosts.lock().await;
        // check if user is hosting a room        
        if hosts.contains_key(&to_user_id) {
            return false;
        }
        let room_id = hosts.get(&from_user_id).cloned();
        let Some(room_id) = room_id else {
            return false;
        };
        hosts.remove(&from_user_id);
        hosts.insert(to_user_id, room_id);
        true
    }
    pub async fn end_game (&self, room_id: RoomId) -> bool {
        let mut rooms = self.rooms.lock().await;
        if let Some(room) = rooms.get_mut(&room_id) {
            room.game_in_progress = false;
            true
        } else {
            false
        }
    }
    pub async fn delete_room(&self, room_id: &RoomId) -> Option<Room> {
        let room = self.rooms.lock().await.remove(room_id)?;
        let host_id = room.host_id;

        self.hosts.lock().await.remove(&host_id);
        Some(room)
    }
}