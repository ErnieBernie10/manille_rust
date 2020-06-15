use crate::player::Player;
use crate::deck::FullDeck;
use crate::input_handler::InputHandler;

pub struct Game<IH: InputHandler> {
    pub players: [Player; 2],
    pub current_player_index: usize,
    pub input_handler: IH
}


impl<IH: InputHandler> Game<IH> {
    pub fn new(input_handler: IH) -> Self {
        let full_deck = FullDeck::generate_new();

        let player1 = Player::new();
        let player2 = Player::new();

        let mut game = Game::<IH> {
            players: [player1, player2],
            current_player_index: 0,
            input_handler
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

    fn next(&mut self) {
        if self.current_player_index >= 1 {
            self.current_player_index = 0;
        } else {
            self.current_player_index += 1;
        }
    }

    pub fn start(&mut self) {
        loop {
            self.input_handler.select_card(self.current_player());
            self.next();
        }
    }
}