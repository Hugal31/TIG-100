use std::io;
use std::io::Write;

use termion::cursor;
use tig_100_game::{Cell, CodeCell};

use term_display::TermDisplay;

#[derive(Debug, Default)]
pub struct Cursor {
    pub cell_x: u16,
    pub cell_y: u16,
    pub row: u16,
    pub column: u16,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor::default()
    }

    pub fn go_right(&mut self) -> bool {
        if (self.column as usize) < CodeCell::MAX_NB_COLUMN - 1 {
            self.column += 1;
            true
        } else {
            false
        }
    }

    pub fn go_left(&mut self) -> bool {
        if self.column > 0 {
            self.column -= 1;
            true
        } else {
            false
        }
    }

    pub fn go_up(&mut self) -> bool {
        if self.row > 0 {
            self.row -= 1;
            self.column = 0;
            true
        } else {
            false
        }
    }

    pub fn go_down(&mut self) -> bool {
        if (self.row as usize) < CodeCell::MAX_NB_ROW - 1 {
            self.row += 1;
            self.column = 0;
            true
        } else {
            false
        }
    }
}

impl TermDisplay for Cursor {
    const TERM_SIZE: (u16, u16) = (1, 1);

    fn display_at<W>(&self, mut w: W, x: u16, y: u16) -> io::Result<()>
    where
        W: Write,
    {
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
