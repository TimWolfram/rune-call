use crate::model::{card::Card, player::Player};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Round {
    pub played_cards: Vec<(Player, Card)>,
}

impl Round {
    pub fn new() -> Round {
        Round {
            played_cards: Vec::new(),
        }
    }
}
