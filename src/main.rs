extern crate termion;
extern crate tig_100;

mod term_display;

use std::io;
use std::io::{stdin, stdout, Write};

use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tig_100::{Cell, CodeCell, Game};

use term_display::TermDisplay;

#[derive(Debug)]
struct Cursor<'a> {
    game: &'a mut Game,
    cell_x: u16,
    cell_y: u16,
    row: u16,
    column: u16,
    buffer: Vec<char>,
}

impl<'a> Cursor<'a> {
    pub fn new(game: &'a mut Game) -> Cursor<'a> {
        Cursor {
            game,
            cell_x: 0,
            cell_y: 0,
            row: 0,
            column: 0,
            buffer: Vec::with_capacity(CodeCell::MAX_NB_COLUMN),
        }
    }

    fn apply_buffer(&mut self) {
        let Cell::Code(ref mut code_cell) = self.game.grid.cells[self.cell_y as usize][self.cell_x as usize];
        if self.buffer.is_empty() {
            code_cell.code[self.row as usize] = None;
        } else if let Ok(instruction) = self.buffer.iter().collect::<String>().parse() {
            code_cell.code[self.row as usize] = Some(instruction);
        }
    }

    fn reset_buffer(&mut self) {
        let Cell::Code(ref code_cell) = self.game.grid.cells[self.cell_y as usize][self.cell_x as usize];
        if let Some(ref instruction) = code_cell.code[self.row as usize] {
            self.buffer = instruction.to_string().chars().collect::<Vec<_>>();
        } else {
            self.buffer.clear();
        }
    }

    pub fn go_right(&mut self) {
        if (self.column as usize) < CodeCell::MAX_NB_COLUMN - 1 {
            self.column += 1;
        }
    }

    pub fn go_left(&mut self) {
        if self.column > 0 {
            self.column -= 1;
        }
    }

    pub fn go_up(&mut self) {
        if self.row > 0 {
            self.apply_buffer();
            self.row -= 1;
            self.column = 0;
            self.reset_buffer();
        }
    }

    pub fn go_down(&mut self) {
        if (self.row as usize) < CodeCell::MAX_NB_ROW - 1 {
            self.apply_buffer();
            self.row += 1;
            self.column = 0;
            self.reset_buffer();
        }
    }

    pub fn type_char(&mut self, c: char) {
        while self.buffer.len() <= self.column as usize {
            self.buffer.push(' ');
        }
        self.buffer[self.column as usize] = c.to_uppercase().next().unwrap();
        self.go_right();
    }

    pub fn remove_char(&mut self) {
        self.go_left();
        self.buffer[self.column as usize] = ' ';
    }

    pub fn delete_char(&mut self) {
        self.buffer[self.column as usize] = ' ';
    }
}

impl<'a> TermDisplay for Cursor<'a> {
    const TERM_SIZE: (u16, u16) = (1, 1);

    fn display_at<W>(&self, mut w: W, x: u16, y: u16) -> io::Result<()>
    where
        W: Write,
    {
        self.game.display_at(&mut w, x, y)?;
        write!(
            w,
            "{}{}",
            cursor::Goto(
                x + self.cell_x * (Cell::TERM_SIZE.0 + 3) + 1,
                y + self.cell_y * (Cell::TERM_SIZE.1 + 3) + self.row + 1,
            ),
            self.buffer.iter().collect::<String>()
        )?;
        write!(
            w,
            "{}",
            cursor::Goto(
                x + self.cell_x * (Cell::TERM_SIZE.0 + 3) + self.column + 1,
                y + self.cell_y * (Cell::TERM_SIZE.1 + 3) + self.row + 1,
            )
        )
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = Game::default();
    let mut cursor = Cursor::new(&mut game);

    refresh(&mut stdout, &cursor).unwrap();

    for evt in stdin.events() {
        let evt = evt.unwrap();
        match evt {
            Event::Key(Key::Ctrl('q')) => break,
            Event::Key(Key::Right) => cursor.go_right(),
            Event::Key(Key::Left) => cursor.go_left(),
            Event::Key(Key::Down) => cursor.go_down(),
            Event::Key(Key::Up) => cursor.go_up(),
            Event::Key(Key::Char(c)) => cursor.type_char(c),
            Event::Key(Key::Backspace) => cursor.remove_char(),
            Event::Key(Key::Delete) => cursor.delete_char(),
            _ => (),
        }
        refresh(&mut stdout, &cursor).unwrap();
    }
}

fn refresh<W>(mut w: W, cursor: &Cursor) -> io::Result<()>
where
    W: Write,
{
    cursor.display_at(&mut w, 1, 1)
        .and_then(|()| w.flush())
}
