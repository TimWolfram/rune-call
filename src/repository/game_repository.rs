use std::{collections::HashMap, sync::atomic::AtomicUsize};
use rocket::tokio::sync::Mutex;

use crate::model::{game::{Game, Room, RoomId}, login::User};

type Map<K, V> = Mutex<HashMap<K, V>>;

pub struct GameRepository {
    games: Map<RoomId, Game>, //stores games by room id
    game_count: AtomicUsize, //generates new ids for games
}

impl Default for GameRepository {
    fn default() -> Self {
        GameRepository {
            games: Mutex::new(HashMap::new()),
            game_count: AtomicUsize::new(1),
        }
    }
}

impl GameRepository{
    pub async fn create_game(&self, room: Room) -> Result<Game, &'static str> {
        let room_id = room.id;
        let mut games = self.games.lock().await;
        let last_game = games.get(&room_id).cloned();
        let game = Game::create(&mut room.clone(), last_game)?;
        games.insert(room_id, game.clone());
        Ok(game)
    }
    pub async fn get_game_from_room(&self, room_id: usize) -> Option<Game> {
        self.games.lock().await.get(&room_id).cloned()
    }
    pub async fn delete_game(&self, game_id: usize) -> Option<Game> {
        let mut games = self.games.lock().await;
        games.remove(&game_id)
    }
    pub async fn update_game(&self, game: Game) -> Option<Game> {
        todo!()
    }
}
