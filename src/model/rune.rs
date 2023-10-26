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
impl Rune {
    pub fn iter() -> impl Iterator<Item = Rune> {
        [Rune::Arcane, Rune::Void, Rune::Life, Rune::Control, Rune::Fire, Rune::Frost].iter().cloned()
    }
    pub fn count() -> usize {
        6
    }
}