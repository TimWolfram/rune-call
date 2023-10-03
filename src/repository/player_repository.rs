use crate::model::Player;

pub struct PlayerRepository {
    pub players: Vec<Player>,
}

impl Default for PlayerRepository {
    fn default() -> Self {
        PlayerRepository {
            players: Vec::new(),
        }
    }
}
