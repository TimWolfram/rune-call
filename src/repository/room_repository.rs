use crate::model::Room;
use std::collections::HashMap;
use rocket::tokio::sync::Mutex;
use std::sync::atomic::AtomicUsize;

type HostPlayerId = usize;
type RoomHosts = HashMap<HostPlayerId, Room>;

pub struct RoomRepository {
    pub rooms: Mutex<RoomHosts>,        //stores rooms by host player id
    pub count: AtomicUsize,             //stores number of rooms
}

impl Default for RoomRepository {
    fn default() -> Self {
        RoomRepository {
            rooms: Mutex::new(HashMap::new()),
            count: AtomicUsize::new(0),
        }
    }
}
trait RoomRepositoryTrait {
    fn get_rooms(&self) -> Vec<Room>;
    fn get_room(&self, host_player_id: HostPlayerId) -> Option<Room>;
    fn delete_room(&self, host_player_id: HostPlayerId) -> Option<Room>;
    fn create_room(&self, host_player_id: HostPlayerId, room: Room) -> Option<Room>;
}