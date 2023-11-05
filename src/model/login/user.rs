use super::Role;

pub type UserId = usize;
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password_hash: String,
    pub nickname: String,
    pub role: Role,
    pub current_room: Option<usize>,
}

impl User {
    pub fn new(id: UserId, username: String, password_hash: String, nickname: String, role: Role) -> Self {
        User{id, username, password_hash, nickname, role, current_room: None}
    }
}