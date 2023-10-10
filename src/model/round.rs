use crate::model::{player::Player, card::Card};

#[derive(Clone)]
#[derive(Debug)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Round {
    pub players: Vec<(Player, Option<Card>)>,
}

impl Round {
    pub fn new(players: Vec<Player>) -> Round {
        let mut players_with_cards:Vec<(Player, Option<Card>)> = Vec::new();
        for player in players {
            players_with_cards.push((player, None));
        }
        Round {
            players: players_with_cards,
        }
    }
}