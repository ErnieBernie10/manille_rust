mod game;
mod player;
mod player_manager;
mod deck;
mod card;
use game::Game;

fn main() {
    // let games: Vec<Game> = create_games(10000);
    let game: Game = Game::new();
    game.start();
}

fn create_games(amount: i32) -> Vec<Game> {
    let mut games = Vec::<Game>::new();
    for i in 0..amount {
        games.push(Game::new());
    }
    games
}