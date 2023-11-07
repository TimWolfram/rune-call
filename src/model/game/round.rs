use crate::model::{game::card::Card, game::player::Player};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Round {
    pub played_cards: Vec<Card>,
    pub round_winner: Option<Player>,
    pub starting_player_index: usize,
}