use std::{collections::HashMap, sync::atomic::AtomicUsize};
use rocket::tokio::sync::Mutex;

use crate::model::{game::{Game, Room}, login::User};

type Map<K, V> = Mutex<HashMap<K, V>>;

pub struct GameRepository {
    games: Map<usize, Game>, //stores games by game id
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
    pub async fn create_game(&self, game: Game, room: Room) -> Option<&Game> {
        todo!()
    }
    pub async fn get_game(&self, game_id: usize) -> Option<&Game> {
        todo!()
    }
    pub async fn delete_game(&self, game_id: usize) -> Option<Game> {
        todo!()
    }
    pub async fn update_game(&self, game: Game) -> Option<Game> {
        todo!()
    }
}
