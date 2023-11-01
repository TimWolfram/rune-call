use crate::model::{game::card::Card, game::player::Player};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Round {
    pub played_cards: Vec<PlayedCard>,
    pub round_winner: Option<Player>,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayedCard {
    pub player: Player,
    pub card: Card,
}