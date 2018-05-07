extern crate termion;
extern crate tig_100_game;

mod cursor;
mod game_manager;
mod term_display;

use std::io::{stdin, stdout};

use termion::raw::IntoRawMode;
use tig_100_game::Game;

use game_manager::GameManager;

fn main() {
    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let game = Game::default();

    GameManager::new(stdin, stdout, game).play().unwrap();
}
