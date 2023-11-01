use crate::model::{game::Player, login::User};
use crate::model::game::Game;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    pub id: usize,
    pub name: String,
    pub password: String,
    pub players: [Option<Player>;4],
    pub host_user_id: usize,
    pub game: Option<Game>,
    pub game_history: Vec<Game
}

impl Room {
    pub fn new(id: usize, name: String, password: String, host_user: &User) -> Self {
        Room {
            id,
            host_user_id: host_user.id,
            name,
            password,
            players: [
                Some(Player::new(host_user,
                    host_user.nickname.clone())),
                    None,
                    None,
                    None],
            game: None,
            game_history: Vec::new(),
        }
    }
}