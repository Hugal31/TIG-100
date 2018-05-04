extern crate termion;
extern crate tig_100;

mod term_display;

use std::io::{stdin, stdout, Write};

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tig_100::Game;

use term_display::TermDisplay;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let game = Game::new();

    game.display_at(&mut stdout, 1, 1).unwrap();
    stdout.flush().unwrap();

    for evt in stdin.events() {
        let evt = evt.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            _ => (),
        }
    }
}
