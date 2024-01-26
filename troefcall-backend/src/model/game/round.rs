use crate::model::game::card::Card;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Round {
    pub played_cards: Vec<Card>,
    pub state: RoundState,
    pub player_starting: usize, //index of player who starts the next round (index in game, not id)
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum RoundState {
    InProgress,
    RoundWon{winner_user_id: usize},
}
impl Round {
    pub fn new(player_starting: usize) -> Self {
        Round {
            played_cards: Vec::new(),
            state: RoundState::InProgress,
            player_starting,
        }
    }
    pub fn set_winner(&mut self, winning_player_id: usize) {
        self.state = RoundState::RoundWon{winner_user_id: winning_player_id};
    }
}