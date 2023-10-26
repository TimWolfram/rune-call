use crate::model::{CreateRoomForm, Game, Room, Player};
use rocket::{tokio::sync::Mutex, State};
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

pub type HostPlayerId = usize;

#[rocket::async_trait]
pub trait RoomApi
where
    Self: Sized + Sync + Send + 'static,
{
    async fn get_rooms<T>(state: &State<Self>) -> Vec<Room>;

    async fn get_room(&self, host_player_id: HostPlayerId) -> Option<Room>;
    async fn create_room(
        &self,
        host_player: Player,
        room_form: CreateRoomForm,
    ) -> Result<Room, &str>;
    async fn delete_room(&self, host_player_id: HostPlayerId) -> Option<Room>;
}

pub struct RoomRepository {
    pub rooms: Mutex<HashMap<HostPlayerId, Room>>, //stores rooms by host player id
    pub games: Mutex<HashMap<usize, Game>>,        //stores games by room id
    pub room_count: AtomicUsize,                   //stores number of rooms
}

impl Default for RoomRepository {
    fn default() -> Self {
        RoomRepository {
            rooms: Mutex::new(HashMap::new()),
            room_count: AtomicUsize::new(0),
            games: Mutex::new(HashMap::new()),
        }
    }
}

#[rocket::async_trait]
impl RoomApi for RoomRepository {
    async fn get_rooms<T>(state: &State<Self>) -> Vec<Room> {
        state.rooms.lock().await.values().cloned().collect()
    }
    async fn get_room(&self, host_player_id: HostPlayerId) -> Option<Room> {
        self.rooms.lock().await.get(&host_player_id).cloned()
    }
    async fn delete_room(&self, host_player_id: HostPlayerId) -> Option<Room> {
        self.rooms.lock().await.remove(&host_player_id)
    }
    async fn create_room(
        &self,
        host_player: Player,
        room_form: CreateRoomForm,
    ) -> Result<Room, &str> {
        if room_form.name.len() < 3 {
            return Err("Room name must be at least 3 characters long!");
        }

        let mut rooms = self.rooms.lock().await;
        if rooms.contains_key(&host_player.id) {
            return Err("Player is already hosting a room!");
        }

        let x = rooms.insert(host_player.id, Room::from_form(room_form, &host_player));
        if x.is_none() {
            //should not be happening since we checked if the player is already hosting a room; just in case
            return Err("Room could not be created! (unknown error)");
        }
        Ok(x.unwrap())
    }
}
