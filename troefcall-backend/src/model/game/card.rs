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
    pub fn to_char(&self) -> char {
        match self {
            Suit::Spades => '♠',
            Suit::Hearts => '♥',
            Suit::Clubs => '♣',
            Suit::Diamonds => '♦',
        }
    }
}
impl ToString for Card{
    fn to_string(&self) -> String {
        let val = match self.value {
            11 => "J".to_string(), //yes this is necessary, "foo" is a str, not a String
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => self.value.to_string(),
        };
        return format!("{}{}", self.suit.to_char(), val);
    }
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