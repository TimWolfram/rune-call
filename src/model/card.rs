use crate::model::Rune;
#[derive(Clone)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
pub struct Card {
    rune : Rune,
    value : u32,
}