
use rocket::tokio::sync::Mutex;

use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

type RoomId = usize;
type Map<K, V> = Mutex<HashMap<K, V>>;

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

use crate::model::login::{UserId, User};

impl RoomRepository {
    // pub async fn get_rooms(&self) -> Vec<Room> {
    //     self.rooms.lock().await.values().cloned().collect()
    // }
    pub async fn get_rooms_paged(&self, start: usize, count: usize) -> Vec<Room> {
        let rooms = self.rooms.lock().await;
        rooms.values()
            .skip(start)
            .take(count)
            .cloned()
            .collect()
    }
    pub async fn get_rooms_public_paged(&self, start: usize, count: usize) -> Vec<Room> {
        let rooms = self.rooms.lock().await;
        rooms.values()
            .filter(|room| room.password.len() == 0)
            .skip(start)
            .take(count)
            .cloned()
            .collect()
    }
    // pub async fn get_room_by_host(&self, host_user_id: UserId) -> Result<Room, &'static str> {
    //     let hosts = self.hosts.lock().await;
    //     let room_id = hosts.get(&host_user_id).cloned()
    //         .ok_or("User is not hosting a room!")?;
    //     let rooms = self.rooms.lock().await;
    //     rooms.get(&room_id).cloned()
    //         .ok_or("Room no longer exists!")
    // }
    pub async fn user_is_host(&self, user_id: usize, room_id: RoomId) -> bool {
        let hosts = self.hosts.lock().await;
        hosts.get(&user_id) == Some(&room_id)
    }
    pub async fn get_room_by_id(&self, room_id: RoomId) -> Result<Room, &'static str> {
        let rooms = self.rooms.lock().await;
        rooms.get(&room_id).cloned().ok_or("Game not found!")
    }
    pub async fn create_room(&self, host_user: &mut User, name: String, password: String) -> Result<Room, &'static str> {
        if host_user.current_room.is_some() {
            return Err("User is already in a room! Leave the room before creating a new one!");
        }
        if name.len() < 3 { // check if name is valid
            return Err("Room name must be at least 3 characters long!");
        }
        let mut hosts = self.hosts.lock().await;
        if hosts.contains_key(&host_user.id) { // check if user is already hosting a room
            return Err("User is already hosting a room!");
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
    pub async fn transfer_host(&self, from_user_id: UserId, to_user_id: UserId) -> bool {
        let mut hosts = self.hosts.lock().await;
        let room_id = hosts.get(&from_user_id).cloned();
        match room_id {
            Some(room_id) => {
                hosts.remove(&from_user_id);
                hosts.insert(to_user_id, room_id);
                true
            }
            None => false,
        }
    }
    pub async fn delete_room(&self, room_id: &RoomId) -> Option<Room> {
        let room = self.rooms.lock().await.remove(room_id)?;
        let host_id = room.host_id;

        self.hosts.lock().await.remove(&host_id);
        Some(room)
    }
}