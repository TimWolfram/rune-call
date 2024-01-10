use crate::model::game::card::Card;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Round {
    pub played_cards: Vec<Card>,
    pub state: RoundState,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum RoundState {
    InProgress,
    RoundWon{winner_index: usize},
}
impl Round {
    pub fn new() -> Self {
        Round {
            played_cards: Vec::new(),
            state: RoundState::InProgress,
        }
    }
    pub fn set_winner(&mut self, winner_index: usize) {
        self.state = RoundState::RoundWon{winner_index};
    }
}