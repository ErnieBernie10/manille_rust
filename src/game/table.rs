use super::player::Player;
use super::deck::FullDeck;

pub struct Game {
    pub players: [Player; 2]
}

impl Game {
    pub fn new() -> Game {

        let _full_deck = FullDeck::generate_new();

        return Game {
            players: [Player::new(), Player::new()]
        };
    }
}