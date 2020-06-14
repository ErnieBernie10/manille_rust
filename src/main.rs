mod game;
mod player;
mod deck;
mod card;
use game::Game;

fn main() {
    // let games: Vec<Game> = create_games(10000);
    let game: Game = Game::new();
    game.start();
}