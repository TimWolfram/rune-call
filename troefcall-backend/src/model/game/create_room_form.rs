#[derive(serde::Deserialize, Debug)]
pub struct CreateRoomForm {
    // #[validate(length(min = 3))]
    pub name: String,
    pub password: String,
}