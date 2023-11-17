use crate::model::login::User;

use super::Card;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Player {
    pub user_id: usize,
    pub name: String,
    #[serde(skip_serializing)] // Room contains a list of all players, and we do not want to send the cards of other players to the client.
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
