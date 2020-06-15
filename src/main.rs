mod game;
mod player;
mod deck;
mod card;
mod input_handler;
use game::Game;
use input_handler::PlayerInputHandler;
use input_handler::InputHandler;

fn main() {
    // let games: Vec<Game> = create_games(10000);
    let player_input_handler = PlayerInputHandler::new();
    let game: Game<PlayerInputHandler> = Game::new(player_input_handler);
    game.start();
}