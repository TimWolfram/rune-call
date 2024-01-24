use crate::model::{game::Player, login::User};
use serde::{Deserialize, Serialize};

pub type RoomId = usize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub password: String, // Serialize password for frontend empty check; hashed room passwords are less sensitive and short-lived
    pub players: [Option<Player>;4],
    pub host_id: usize,
    pub game_in_progress: bool,
}

impl Room {
    pub fn new(id: usize, name: String, password: String, host_user: &User) -> Self {
        Room {
            id,
            host_id: host_user.id,
            name,
            password,
            players: [
                Some(Player::from(host_user)),
                    None,
                    None,
                    None],
            game_in_progress: false,
        }
    }
}