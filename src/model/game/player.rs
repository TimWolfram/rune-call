use crate::model::login::User;

use super::Card;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Player {
    pub user_id: usize,
    pub name: String,
    pub current_cards: Vec<Card>,
}
impl Player {
    pub fn new(user: &User, name: String) -> Self {
        Player {
            user_id: user.id,
            name,
            current_cards: Vec::new(),
        }
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}