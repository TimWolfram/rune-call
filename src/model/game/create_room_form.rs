#[derive(serde::Deserialize, Debug)]
pub struct CreateRoomForm {
    pub name: String,
    pub password: String,
}