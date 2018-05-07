use std::io;
use std::io::{Read, Write};

use termion::event::{Event, Key};
use termion::input::{Events, TermRead};
use tig_100_game::{Cell, CodeCell, Game};

use cursor::Cursor;
use term_display::TermDisplay;

/// Manage and display a [`Game`].
pub struct GameManager<R, W>
where
    R: Read,
    W: Write,
{
    cursor: Cursor,
    game: Game,
    events: Events<R>,
    output: W,
    buffer: Vec<char>,
}

impl<R, W> GameManager<R, W>
where
    R: Read,
    W: Write,
{
    pub fn new(input: R, output: W, game: Game) -> GameManager<R, W> {
        GameManager {
            cursor: Cursor::new(),
            game,
            events: input.events(),
            output,
            buffer: Vec::with_capacity(CodeCell::MAX_NB_COLUMN),
        }
    }

    pub fn play(&mut self) -> io::Result<()> {
        self.refresh()?;

        while let Some(evt) = self.events.next() {
            let evt = evt.unwrap();
            match evt {
                Event::Key(Key::Ctrl('q')) => break,
                Event::Key(Key::Right) => self.cursor_right()?,
                Event::Key(Key::Left) => self.cursor_left()?,
                Event::Key(Key::Down) => self.cursor_down()?,
                Event::Key(Key::Up) => self.cursor_up()?,
                Event::Key(Key::Char(c)) => self.type_char(c)?,
                //                Event::Key(Key::Backspace) => cursor.remove_char(),
                //                Event::Key(Key::Delete) => cursor.delete_char(),
                _ => (),
            }
            self.output.flush()?;
        }

        Ok(())
    }

    fn cursor_up(&mut self) -> io::Result<()> {
        self.save_code();
        if self.cursor.go_up() {
            self.reset_buffer();
            self.load_code();
            let x = self.cursor.cell_x;
            let y = self.cursor.cell_y;
            self.refresh_cell(x, y)
                .and_then(|()| self.refresh_cursor_and_buffer())
        } else {
            Ok(())
        }
    }

    fn cursor_down(&mut self) -> io::Result<()> {
        self.save_code();
        if self.cursor.go_down() {
            self.reset_buffer();
            self.load_code();
            let x = self.cursor.cell_x;
            let y = self.cursor.cell_y;
            self.refresh_cell(x, y)
                .and_then(|()| self.refresh_cursor_and_buffer())
        } else {
            Ok(())
        }
    }

    fn cursor_left(&mut self) -> io::Result<()> {
        if self.cursor.go_left() {
            self.refresh_cursor_and_buffer()
        } else {
            Ok(())
        }
    }

    fn cursor_right(&mut self) -> io::Result<()> {
        if self.cursor.go_right() {
            self.refresh_cursor_and_buffer()
        } else {
            Ok(())
        }
    }

    fn type_char(&mut self, c: char) -> io::Result<()> {
        while self.buffer.len() <= (self.cursor.column as usize) {
            self.buffer.push(' ');
        }
        self.buffer[self.cursor.column as usize] = c.to_uppercase().next().unwrap();
        self.cursor.go_right();
        self.refresh_cursor_and_buffer()
    }

    fn save_code(&mut self) {
        let code = self.buffer.iter().collect::<String>();
        let Cell::Code(ref mut cell) = self.game.grid.cells[self.cursor.cell_y as usize][self.cursor.cell_x as usize];
        cell.code[self.cursor.row as usize] = code.parse().ok();
    }

    fn load_code(&mut self) {
        let Cell::Code(ref cell) = self.game.grid.cells[self.cursor.cell_y as usize][self.cursor.cell_x as usize];
        self.buffer = cell.code[self.cursor.row as usize].as_ref()
            .map(|i| i.to_string().chars().collect())
            .unwrap_or(Vec::with_capacity(CodeCell::MAX_NB_COLUMN));

    }

    fn reset_buffer(&mut self) {
        self.buffer.clear();
    }

    // TODO Display buffer
    fn refresh(&mut self) -> io::Result<()> {
        self.game
            .display_at(&mut self.output, 1, 1)
            .and_then(|()| {
                self.buffer.display_at(
                    &mut self.output,
                    self.cursor.cell_x * (Cell::TERM_SIZE.0 + 3) + self.cursor.column + 1,
                    self.cursor.cell_y * (Cell::TERM_SIZE.1 + 3) + 1,
                )
            })
            .and_then(|()| self.cursor.display_at(&mut self.output, 1, 1))
            .and_then(|()| self.output.flush())
    }

    fn refresh_cursor_and_buffer(&mut self) -> io::Result<()> {
        self.buffer
            .display_at(
                &mut self.output,
                self.cursor.cell_x * (Cell::TERM_SIZE.0 + 3) + 2,
                self.cursor.cell_y * (Cell::TERM_SIZE.1 + 3) + self.cursor.row + 2,
            )
            .and_then(|()| self.cursor.display_at(&mut self.output, 1, 1))
    }

    fn refresh_cell(&mut self, cell_x: u16, cell_y: u16) -> io::Result<()> {
        self.game.grid.cells[cell_y as usize][cell_x as usize].display_at(
            &mut self.output,
            1 + cell_x * (Cell::TERM_SIZE.0 + 3),
            1 + cell_y * (Cell::TERM_SIZE.1 + 3),
        )
    }
}
