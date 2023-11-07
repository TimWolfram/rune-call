use std::collections::HashMap;
use rocket::tokio::sync::Mutex;

use crate::model::game::{Game, Room, RoomId};

type Map<K, V> = Mutex<HashMap<K, V>>;

pub struct GameRepository {
    games: Map<RoomId, Game>, //stores games by room id
}

impl Default for GameRepository {
    fn default() -> Self {
        GameRepository {
            games: Mutex::new(HashMap::new()),
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
    pub async fn get_game_from_room(&self, room_id: usize) -> Result<Game, &'static str> {
        self.games.lock().await
            .get(&room_id).cloned()
            .ok_or("Game not found!")
    }
    pub async fn update_game(&self, room_id: usize, game: Game) -> () {
        let mut games = self.games.lock().await;
        games.insert(room_id, game);
    }
}
