use crate::model::{game::Player, login::User};

impl From<User> for Player{
    fn from(user: User) -> Self {
        Player {
            user_id: user.id,
            name: user.nickname,
            current_cards: Vec::new(),
        }
    }
}