use crate::model::login::User;

use super::Card;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Player {
    pub user_id: usize,
    pub name: String,
    pub current_cards: Vec<Card>,
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl From<&User> for Player {
    fn from(user: &User) -> Self {
        Player {
            user_id: user.id,
            name: user.nickname.clone(),
            current_cards: Vec::new(),
        }
    }
}
