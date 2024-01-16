use rocket::http::Status;
use rocket::http::ext::IntoCollection;
use rocket::tokio::sync::Mutex;

use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

type RoomId = usize;
type Map<K, V> = Mutex<HashMap<K, V>>;
type Error<'a> = (Status, &'a str);

use crate::controller::password;
use crate::model::game::Room;
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
    pub fn test_repo(user_repo: UserRepository) -> Self {
        let mut room_amt: usize = 30;
        let users:HashMap<usize, User> = user_repo.users.into_inner();
        if users.len() < room_amt {
            println!("Not enough users in user repo to create test {room_amt} rooms!");
            room_amt = users.len();
        }
        let rooms_map = HashMap::from([
            (0, Room::new(0, "Room 0 with a super long name for some reason, good luck displaying this properly".to_string(), "asdf".to_string(), users.get(&0).unwrap()))
        ]);
        let hosts_map = HashMap::from([
            (0, 0),
        ]);
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
    pub async fn get_rooms_paged(&self, start: usize, count: usize) -> Vec<Room> {
        let mut rooms = self.rooms.lock().await.values()
            .filter(|room| room.game_in_progress == false)
            .cloned()
            .collect::<Vec<Room>>();
        rooms.sort_by_key(|room| room.id);
        rooms.iter().skip(start).take(count).cloned().collect()
    }
    /// Returns a vector of all public rooms in the repository, sorted by id. Uses pagination.
    pub async fn get_rooms_public_paged(&self, start: usize, count: usize) -> Vec<Room> {
        let mut rooms = self.rooms.lock().await.values()
            .filter(|room| room.game_in_progress == false & (room.password.len() == 0))
            .cloned()
            .collect::<Vec<Room>>();
        rooms.sort_by_key(|room| room.id);
        rooms.iter().skip(start).take(count).cloned().collect()
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
        rooms.get(&room_id).cloned().ok_or((Status::Unauthorized, "Game not found!"))
    }
    pub async fn create_room<'a> (&self, host_user: &mut User, name: String, password: String) -> Result<Room, Error<'a>> {
        if host_user.current_room.is_some() {
            return Err((Status::Unauthorized, "User is already in a room! Leave the room before creating a new one!"));
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
        let room_id = hosts.get(&from_user_id).cloned();
        match room_id {
            None => false,
            Some(room_id) => {
                hosts.remove(&from_user_id);
                hosts.insert(to_user_id, room_id);
                true
            }
        }
    }
    pub async fn delete_room(&self, room_id: &RoomId) -> Option<Room> {
        let room = self.rooms.lock().await.remove(room_id)?;
        let host_id = room.host_id;

        self.hosts.lock().await.remove(&host_id);
        Some(room)
    }
}