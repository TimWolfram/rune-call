#[derive(serde::Deserialize, Debug)]
pub struct CreateRoomForm {
    pub host_id: i32,
    pub name: String,
    pub password: String,
}