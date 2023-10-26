use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerToken {
    pub player_id: usize,
    pub exp: usize,
}