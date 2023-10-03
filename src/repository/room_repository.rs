use crate::model::{Room, Player};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct RoomRepository {
    rooms: HashMap<usize, Room>,
    count: AtomicUsize,
}

impl Default for RoomRepository {
    fn default() -> Self {
        RoomRepository {
            rooms: HashMap::new(),
            count: AtomicUsize::new(0),
        }
    }
}

impl RoomRepository {
    pub fn create_room(&mut self, host_player: &Player, room_name: &str, room_pwd: &str) -> &Room {
        self.count.fetch_add(1, Ordering::SeqCst);
        let id = self.count.load(Ordering::SeqCst);
        let room = Room::create(id, host_player, room_name, room_pwd);
        self.rooms.insert(id, room);
        self.rooms.get(&id).unwrap()
    }

    pub fn get_room(&self, id: usize) -> Option<&Room> {
        self.rooms.get(&id)
    }

    pub fn get_room_mut(&mut self, id: usize) -> Option<&mut Room> {
        self.rooms.get_mut(&id)
    }

    pub fn delete_room(&mut self, id: usize) -> bool {
        self.rooms.remove(&id).is_some()
    }
}