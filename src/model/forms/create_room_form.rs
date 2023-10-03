#[derive(FromForm)]
pub struct CreateRoomForm {
    pub host_player_id: usize,
    pub name: String,
    pub password: String,
    pub max_players: u8,
    pub max_rounds: u8,
    pub max_points: u8,
    pub is_public: bool,
}