use crate::model::{player::Player, game::Game};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Room {
    pub id: usize,
    pub name: String,
    pub password: String,
      
    pub current_players: Vec<Player>,
    pub played_games: Vec<Game>,
}

impl Room {    
    pub fn create(id: usize, host_player: &Player, room_name: &str, room_pwd: &str) -> Room {
        Room {
            id: id,
            name: room_name.to_string(),
            current_players: vec![host_player.clone()],
            played_games: Vec::new(),
            password: room_pwd.to_string(),
        }
    }
}