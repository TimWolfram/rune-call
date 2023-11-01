use crate::model::{game::card::Card, game::player::Player};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Round {
    pub played_cards: Vec<(Player, Card)>,
    pub round_winner: Option<Player>,
}

impl Round {
    pub fn new() -> Round {
        Round {
            played_cards: Vec::new(),
            round_winner: None,
        }
    }
}