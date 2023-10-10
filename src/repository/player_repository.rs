use std::collections::HashMap;

use crate::model::Player;

pub struct PlayerRepository {
    pub players: HashMap<usize, Player>,
}

impl Default for PlayerRepository {
    fn default() -> Self {
        PlayerRepository {
            // players: vec![Player{name: "test".to_string(), player_id: 0}]
            players: HashMap::from([
                (0, Player{name: "test".to_string(), id: 0})
            ])
        }
    }
}
