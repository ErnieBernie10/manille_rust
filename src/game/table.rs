use super::player::Player;
use super::deck::FullDeck;

pub struct Game {
    pub players: [Player; 2]
}

impl Game {
    pub fn new() -> Game {
        let full_deck = FullDeck::generate_new();

        let mut game = Game {
            players: [Player::new(), Player::new()]
        };
        game.distribute_cards(full_deck);

        game
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
}