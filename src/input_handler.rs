use crate::player::Player;
use std::io::{stdin,stdout,Write};

pub trait InputHandler {
    fn new() -> Self;
    fn select_card(&self, player: &Player);
}

pub struct PlayerInputHandler {  }

impl InputHandler for PlayerInputHandler {
    fn new() -> Self {
        PlayerInputHandler {
        }
    }

    fn select_card(&self, player: &Player) {
        let mut s = String::new();
        println!("{}", player.to_string());
        println!("Pick a card to play");
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        println!("{}", s);
    }
}
