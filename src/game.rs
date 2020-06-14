use crate::player::Player;
use crate::deck::FullDeck;
use std::io::{stdin,stdout,Write};

pub struct Game {
    pub players: [Player; 2],
    pub current_player_index: usize
}

impl Game {
    pub fn new() -> Game {
        let full_deck = FullDeck::generate_new();

        let player1 = Player::new();
        let player2 = Player::new();

        let mut game = Game {
            players: [player1, player2],
            current_player_index: 0
        };
        game.distribute_cards(full_deck);

        game
    }

    fn current_player(&self) -> &Player {
        &self.players[self.current_player_index]
    }

    fn distribute_cards(&mut self, mut deck: FullDeck) {
        // TODO : Implement actual correct way of distribution

        for i in 0..deck.len() {
            let card = deck.take();
            if i % 2 == 0 {
                self.players[0].give_card(card);
            } else {
                self.players[1].give_card(card);
            }
        }
    }

    pub fn start(&self) {
        let mut s = String::new();
        loop {
            println!("{}", self.current_player().to_string());
            println!("Pick a card to play\n");
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            println!("{}", s);
            break;
        }
    }
}