use crate::model::game::{Card, Player, Round, Suit};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    pub players: [Player; 4],
    pub played_rounds: Vec<Round>,
    pub state: GameState,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum GameState {
    Starting {
        #[serde(skip)] 
        remaining_deck: Vec<Card>
    }, // starting player picks tjall from first 5 cards, then the rest of the cards are dealt
    Playing { current_round: Round, tjall: Suit },
    Finished { winners: [Player; 2], reason: EndGameReason },
}
#[derive(Clone, Serialize, Deserialize)]
pub enum EndGameReason {
    Forfeit { player: Player },
    Score { score: [usize; 2] },
}
