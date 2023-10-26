use std::collections::HashMap;

use crate::model::player::Player;
use serde::{Deserialize, Serialize};

pub use super::{game::PlayerTeams, CreateRoomForm, Rune};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Room {
    pub name: String,
    pub password: String,
    pub player_teams: PlayerTeams,
    pub players: Vec<Player>,
    pub host_player_id: usize,
}

impl Room {
    pub fn from_form(form: CreateRoomForm, host_player: &Player) -> Room {
        let host_player = host_player;
        let room_name: &str = &form.name;
        let room_pwd: &str = &form.password;
        Room {
            name: room_name.to_string(),
            password: room_pwd.to_string(),
            player_teams: HashMap::from([(0, vec![host_player.id])]),
            players: vec![host_player.clone()],
            host_player_id: host_player.id,
        }
    }
}
