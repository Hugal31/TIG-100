use std::fmt;

use instructions::{Instruction, Port};

#[derive(Clone, Debug)]
pub enum Cell {
    Code(CodeCell),
}

impl From<CodeCell> for Cell {
    fn from(code_cell: CodeCell) -> Cell {
        Cell::Code(code_cell)
    }
}

#[derive(Clone, Debug, Default)]
pub struct CodeCell {
    pub acc: i16,
    pub back: i16,
    pub status: Status,
    pub code: [Option<Instruction>; CodeCell::MAX_NB_ROW],
    pub last: Option<Port>
}

impl CodeCell {
    pub const MAX_NB_COLUMN: usize = 19;
    pub const MAX_NB_ROW: usize = 15;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Idle,
    Read,
    Write,
}

impl Default for Status {
    fn default() -> Status {
        Status::Idle
    }
}

// TODO: Be sure about this implementation
impl From<Status> for &'static str {
    fn from(status: Status) -> &'static str {
        use self::Status::*;
        
        match status {
            Idle => "IDLE",
            Read => "READ",
            Write => "WRITE",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.clone().into())
    }
}
