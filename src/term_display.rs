use std::io::{Result, Write};

use termion::clear;
use termion::cursor;

use tig_100_game::*;

const CELL_WIDTH: u16 = 27;
const CELL_HEIGHT: u16 = 17;

/// A trait representing objects that can be displayed.
pub trait TermDisplay {
    /// The size taken at screen.
    const TERM_SIZE: (u16, u16);
    /// Display on the screen
    fn display_at<W>(&self, w: W, x: u16, y: u16) -> Result<()>
    where
        W: Write;
}

impl TermDisplay for Vec<char> {
    const TERM_SIZE: (u16, u16) = (1, 1);

    fn display_at<W>(&self, mut w: W, x: u16, y: u16) -> Result<()>
    where
        W: Write,
    {
        write!(w, "{}", cursor::Goto(x, y))?;
        for c in self {
            let mut buf = [0; 4];
            w.write_all(c.encode_utf8(&mut buf).as_bytes())?;
        }

        Ok(())
    }
}

impl TermDisplay for Game {
    const TERM_SIZE: (u16, u16) = Grid::TERM_SIZE;

    fn display_at<W>(&self, mut w: W, x: u16, y: u16) -> Result<()>
    where
        W: Write,
    {
        write!(w, "{}", clear::All)?;
        self.grid.display_at(w, x, y)
    }
}

impl TermDisplay for Grid {
    const TERM_SIZE: (u16, u16) = (
        Cell::TERM_SIZE.0 * Grid::WIDTH as u16 + 3 * (Grid::WIDTH as u16 - 1),
        Cell::TERM_SIZE.1 * Grid::HEIGHT as u16 + 3 * (Grid::HEIGHT as u16 - 1),
    );

    fn display_at<W>(&self, mut w: W, x: u16, y: u16) -> Result<()>
    where
        W: Write,
    {
        write!(w, "{}", cursor::Goto(x, y))?;

        for (line_nb, line) in self.cells.iter().enumerate() {
            for (row_nb, cell) in line.iter().enumerate() {
                cell.display_at(
                    &mut w,
                    x + (line_nb as u16) * (Cell::TERM_SIZE.0 + 3),
                    y + (row_nb as u16) * (Cell::TERM_SIZE.1 + 3),
                )?;
            }
        }

        Ok(())
    }
}

impl TermDisplay for Cell {
    const TERM_SIZE: (u16, u16) = (CELL_WIDTH, CELL_HEIGHT);

    fn display_at<W>(&self, w: W, x: u16, y: u16) -> Result<()>
    where
        W: Write,
    {
        match &self {
            Cell::Code(code_cell) => code_cell.display_at(w, x, y),
        }
    }
}

impl TermDisplay for CodeCell {
    const TERM_SIZE: (u16, u16) = Cell::TERM_SIZE;

    fn display_at<W>(&self, mut w: W, x: u16, y: u16) -> Result<()>
    where
        W: Write,
    {
        // Top border
        write!(
            w,
            "{}╔{:═>3$}{:═>4$}",
            cursor::Goto(x, y),
            "╤",
            "╗",
            CodeCell::MAX_NB_COLUMN + 1,
            CELL_WIDTH as usize - CodeCell::MAX_NB_COLUMN - 2
        )?;

        // Lines of codes
        for (line_nb, code) in self.code.iter().enumerate() {
            write!(w, "{}║", cursor::Goto(x, y + 1 + line_nb as u16))?;
            if let Some(code) = code {
                write!(
                    w,
                    "{: <1$}",
                    code.to_string(),
                    (CodeCell::MAX_NB_COLUMN) as usize
                )?;
            } else {
                write!(w, "{: <1$}", "", (CodeCell::MAX_NB_COLUMN) as usize)?;
            }

            match line_nb {
                0 => w.write_all("│ ACC ║".as_bytes()),
                1 => write!(w, "│{: ^5}║", self.acc),
                3 => w.write_all("│ BAK ║".as_bytes()),
                4 => write!(w, "│{: ^5}║", self.back),
                6 => w.write_all("│ LAST║".as_bytes()),
                7 => write!(
                    w,
                    "│{: ^5}║",
                    self.last.map(|l| l.into()).unwrap_or("N/A")
                ),
                9 => w.write_all("│ MODE║".as_bytes()),
                10 => write!(w, "│ {: ^4}║", self.status),
                12 => w.write_all("│ IDLE║".as_bytes()),
                13 => w.write_all("│  0% ║".as_bytes()),
                2 | 5 | 8 | 11 => w.write_all("├─────╢".as_bytes()),
                _ => w.write_all("│     ║".as_bytes()),
            }?;
        }

        // Bottom border
        write!(
            w,
            "{}╚{:═>3$}{:═>4$}",
            cursor::Goto(x, y + CELL_HEIGHT - 1),
            "╧",
            "╝",
            CodeCell::MAX_NB_COLUMN + 1,
            CELL_WIDTH as usize - CodeCell::MAX_NB_COLUMN - 2
        )
    }
}
