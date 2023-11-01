use rand::seq::SliceRandom;

use crate::model::game::{Card, Suit};

impl Card {
    pub fn generate_deck() -> Vec<Card> {
        let mut deck = Vec::new();
        Suit::iter().for_each(|suit| {
            (2..=14).for_each(|value| { //11: jack, 12: queen, 13: king, 14: ace
                deck.push(Card {
                    value,
                    suit,
                });
            });
        });
        assert_eq!(deck.len(), 52, "Deck should have 52 cards!");
        
        //shuffle
        deck.shuffle(&mut rand::thread_rng());
        deck
    }
}

