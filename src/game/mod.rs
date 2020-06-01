mod player;
use player::Player;


pub struct Game {
    pub players: [Player; 2]
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: [Player::new(), Player::new()]
        }
    }
}