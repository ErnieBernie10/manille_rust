use std::collections::VecDeque;
use crate::card::Card;
use rand::thread_rng;
use rand::seq::SliceRandom;

const SUITS: [&str; 4] = ["spades", "diamonds", "clubs", "hearts"];
const RANKS: [&str; 8] = ["A", "7", "8", "9", "10", "J", "Q", "K"];

pub struct FullDeck {
    cards: VecDeque<Card>
}

pub struct Hand {
    cards: Vec<Card>
}

impl FullDeck {

    fn new() -> FullDeck {
        FullDeck { cards: VecDeque::<Card>::new() }
    }

    // TODO : Try to optimize or find a better way to handle this
    pub fn generate_new() -> FullDeck {
        let mut d = FullDeck::new();
        let mut cs = Vec::new();

        for suit in &SUITS {
            for rank in &RANKS {
                let c = Card::new(String::from(*suit), String::from(*rank));
                cs.push(c);
            }
        }
        cs.shuffle(&mut thread_rng());
        d.cards = VecDeque::from(cs);
        d
    }

    pub fn take(&mut self) -> Card {
        self.cards.remove(0).unwrap()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::<Card>::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}