use std::collections::HashMap;
use rocket::tokio::sync::Mutex;
use crate::{model::Player, password};

pub struct PlayerRepository {
    pub players: Mutex<HashMap<usize, Player>>,
}

impl Default for PlayerRepository {
    fn default() -> Self {
        PlayerRepository {
            players: Mutex::new(HashMap::new()),
        }
    }
}

impl PlayerRepository {
    fn test() -> Self {
        PlayerRepository {
            players: Mutex::new(HashMap::from([
                (0, Player::new("test", password::hash_password("testpassw0rd123").unwrap().as_str()),),
                (1, Player::new("test2", password::hash_password("testpassw0rd123").unwrap().as_str()),),
            ]))
        }
    }
    pub async fn get(&self, id: usize) -> Option<Player> {
        self.players.lock().await.get(&id).cloned()
    }
}
