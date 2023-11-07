use std::str::FromStr;

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit : Suit,
    pub value : u8,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl Suit {
    pub fn iter() -> impl Iterator<Item = Suit> {
        [Suit::Spades, 
        Suit::Hearts, 
        Suit::Clubs, 
        Suit::Diamonds].into_iter()
    }
    // pub fn to_char(&self) -> char {
    //     match self {
    //         Suit::Spades => '♠',
    //         Suit::Hearts => '♥',
    //         Suit::Clubs => '♣',
    //         Suit::Diamonds => '♦',
    //     }
    // }
    // pub fn to_str(&self) -> &'static str {
    //     match self {
    //         Suit::Spades => "Spades",
    //         Suit::Hearts => "Hearts",
    //         Suit::Clubs => "Clubs",
    //         Suit::Diamonds => "Diamonds",
    //     }
    // }
}
impl<'r> rocket::request::FromParam<'r> for Suit {
    type Error = &'static str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        if param.starts_with("S") | param.starts_with("s") | param.starts_with("♠") {
            return Ok(Suit::Spades);
        }
        if param.starts_with("H") | param.starts_with("h") | param.starts_with("♥"){
            return Ok(Suit::Hearts);
        }
        if param.starts_with("C") | param.starts_with("c") | param.starts_with("♣"){
            return Ok(Suit::Clubs);
        }
        if param.starts_with("D") | param.starts_with("d") | param.starts_with("♦"){
            return Ok(Suit::Diamonds);
        }
        Err("Invalid suit!")
    }
}