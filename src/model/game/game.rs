use serde::{Serialize, Deserialize};
use crate::model::game::{Round, Suit, Player, Card};

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub players: [Player;4],
    pub played_rounds: Vec<Round>,
    pub game_state: GameState,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GameState {
    Starting{remaining_deck: Vec<Card>},// starting player picks tjall from first 5 cards, then the rest of the cards are dealt
    Playing{current_round: Round, tjall: Suit},
    Finished{winners: [Player;2]},
}