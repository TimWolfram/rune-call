#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum Rune {
    Arcane,
    Void,
    Life,
    Control,
    Fire,
    Frost,
}