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