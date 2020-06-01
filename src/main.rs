mod game;
use game::table::Game;

fn main() {
    let games: Vec<Game> = create_games(10000);
}

fn create_games(amount: i32) -> Vec<Game> {
    let mut games = Vec::<Game>::new();
    for i in 0..amount {
        games.push(Game::new());
    }
    games
}