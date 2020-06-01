use super::card::Card;

const SUITS: [&str; 4] = ["spades", "diamonds", "clubs", "hearts"];
const RANKS: [&str; 8] = ["A", "7", "8", "9", "10", "J", "Q", "K"];

pub struct Deck {
    pub cards: Vec<Card>
}

pub struct FullDeck {
    pub deck: Deck
}

pub struct Hand {

}

impl FullDeck {

    fn new() -> FullDeck {
        FullDeck { deck: Deck { cards: Vec::<Card>::new() } }
    }

    pub fn generate_new() -> FullDeck {
        let mut d = FullDeck::new();

        for suit in &SUITS {
            for rank in &RANKS {
                let c = Card::new(String::from(*suit), String::from(*rank));
                d.deck.cards.push(c)
            }
        }
        d
    }
}