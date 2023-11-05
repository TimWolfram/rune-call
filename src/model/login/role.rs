#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum Role{
    Admin,
    Player,
    // Banned{reason: String, banned_by: PlayerId, banned_at: PrimitiveDateTime, banned_until: PrimitiveDateTime},
}