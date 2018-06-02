#[macro_use]
extern crate failure;
extern crate termion;
extern crate tig_100_game;

pub mod errors;

mod cursor;
mod game_manager;
mod term_display;

pub use errors::*;

use std::io::{stdin, stdout};

use termion::raw::IntoRawMode;
use tig_100_game::Game;

use game_manager::GameManager;

pub fn run() -> Result<()> {
    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();
    let game = Game::default();

    GameManager::new(stdin, stdout, game).play()
}
