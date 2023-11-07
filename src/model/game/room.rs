use crate::model::{game::Player, login::User};
use serde::{Deserialize, Serialize};

pub type RoomId = usize;

#[derive(Serialize, Deserialize, Clone)]

pub struct Room {
    pub id: RoomId,
    pub name: String,
    // we do not skip serializing the password here because we need to check if it's empty on the front end; also, because rooms are do not last as long as users and are less sensitive, we can afford to send the hashed password over the network.
    pub password: String,
    pub players: [Option<Player>;4],
    pub host_id: usize,
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
        }
    }
}