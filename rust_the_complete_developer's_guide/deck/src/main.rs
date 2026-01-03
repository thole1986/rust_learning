
use core::num;

use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {

    // classmethod for other programming languages.
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamond"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // let deck = Deck {cards: Vec::new()};
        // The same above just different syntax

        // Implicit return -> not use semicolon ';' at the end
        Deck {
            cards
        }
    }

    // This is the method of class Deck
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        ) // implicit return
    }
}

fn main() {
    // Create empty list to contain list of deck objects.
    let mut deck = Deck::new();
    // deck.shuffle();

    // Need to add error handling!!!
    let cards = deck.deal(3);

    println!("Heres your hand: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);
}
