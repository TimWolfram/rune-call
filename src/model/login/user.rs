use super::Role;

pub type UserId = usize;
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password_hash: String,
    pub nickname: String,
    pub role: Role,
}