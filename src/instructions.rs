use std::fmt;

/// Represents an instruction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Label {
        name: String,
        instruction: Option<Box<Instruction>>,
    },
    Nop,
    Mov {
        src: Source,
        dst: Destination,
    },
    Swp,
    Sav,
    Add(Source),
    Sub(Source),
    Neg,
    Jmp(String),
    Jez(String),
    Jnz(String),
    Jgz(String),
    Jlz(String),
    Jro(Source),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match &self {
            Label { name, instruction } => if let Some(instruction) = instruction {
                write!(f, "{}: {}", name, instruction)
            } else {
                write!(f, "{}:", name)
            },
            Nop => f.write_str("NOP"),
            Mov { src, dst } => write!(f, "MOV {} {}", src, dst),
            Swp => f.write_str("SWP"),
            Sav => f.write_str("SAV"),
            Add(src) => write!(f, "ADD {}", src),
            Sub(src) => write!(f, "SUB {}", src),
            Neg => f.write_str("NEG"),
            Jmp(l) => write!(f, "JMP {}", l),
            Jez(l) => write!(f, "JEZ {}", l),
            Jnz(l) => write!(f, "JNZ {}", l),
            Jgz(l) => write!(f, "Jgz {}", l),
            Jlz(l) => write!(f, "Jlz {}", l),
            Jro(s) => write!(f, "JRO {}", s),
        }
    }
}

/// Represent the source of a value. Can be a direct value, a register or a port.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Source {
    Destination(Destination),
    Value(i16),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Source::*;

        match &self {
            Destination(d) => d.fmt(f),
            Value(i) => i.fmt(f),
        }
    }
}

/// Represent a destination
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Destination {
    Acc,
    Nil,
    Port(Port),
}

// TODO: Be sure about this implementation
impl From<Destination> for &'static str {
    fn from(dst: Destination) -> &'static str {
        use self::Destination::*;
        
        match dst {
            Acc => "ACC",
            Nil => "NIL",
            Port(p) => p.into(),
        }
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.clone().into())
    }
}

/// A port from which read or write.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Port {
    Any,
    Down,
    Last,
    Left,
    Up,
    Right,
}

// TODO: Be sure about this implementation
impl From<Port> for &'static str {
    fn from(port: Port) -> &'static str {
        use self::Port::*;
        
        match port {
            Any => "ANY",
            Down => "DOWN",
            Last => "LAST",
            Left => "LEFT",
            Up => "UP",
            Right => "RIGHT",
        }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.clone().into())
    }
}
