
use rocket::{tokio::sync::Mutex, State};

use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

type RoomId = usize;
type Map<K, V> = Mutex<HashMap<K, V>>;

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
use crate::model::game::CreateRoomForm;
impl RoomRepository {
    pub async fn get_rooms(state: &State<Self>) -> Vec<Room> {
        state.rooms.lock().await.values().cloned().collect()
    }
    
    pub async fn get_rooms_paged(state: &State<Self>, start: usize, count: usize) -> Vec<Room> {
        let rooms = state.rooms.lock().await;
        rooms.values()
            .skip(start)
            .take(count)
            .cloned()
            .collect()
    }

    pub async fn get_room_by_host(&self, host_user_id: &UserId) -> Result<Room, &str> {
        let hosts = &self.hosts.lock().await;
        let room_id = hosts.get(host_user_id).cloned()
            .ok_or("User is not hosting a room!")?;
        let rooms = &self.rooms.lock().await;
        rooms.get(&room_id).cloned()
            .ok_or("Room no longer exists!")
    }
    pub async fn get_room_by_id(&self, room_id: &RoomId) -> Option<Room> {
        self.rooms.lock().await.get(room_id).cloned()
    }
    pub async fn delete_room(&self, room_id: &RoomId) -> Result<Room, &str> {
        let room = self.rooms.lock().await.remove(room_id).ok_or("Room does not exist!")?;
        let host_id = room.host_user_id;
        self.hosts.lock().await.remove(&host_id);
        Ok(room)
    }
    pub async fn create_room(&self, host_user: &User, room_form: CreateRoomForm) -> Result<Room, &str> {
        // check if name is valid
        if room_form.name.len() < 3 {
            return Err("Room name must be at least 3 characters long!");
        }
        let mut hosts = self.hosts.lock().await;
        // check if user is already hosting a room
        if hosts.contains_key(host_id) {
            return Err("User is already hosting a room!");
        }
        let room_id = self.room_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let name = room_form.name;
        let password = room_form.password;
        let new_room = Room::new(room_id, name, password, host_user);
        let mut rooms = self.rooms.lock().await;
        hosts.insert(host_id, room_id);
        rooms.insert(room_id, new_room.clone());
        Ok(new_room)
    }
}