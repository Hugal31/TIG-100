use std::fmt;
use std::str::FromStr;

use errors::*;

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

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(column_pos) = s.find(':') {
            let name = &s[0..column_pos].trim();
            let remaining = &s[column_pos + 1..].trim();

            if name.is_empty() {
                return Err(ParseError::InvalidLabel);
            }

            let instruction = if !remaining.is_empty() {
                Some(Box::new(remaining.parse()?))
            } else {
                None
            };

            Ok(Instruction::Label {
                name: name.to_string(),
                instruction,
            })
        } else {
            let mut splitted = s.split_whitespace();
            let op_code = splitted.next().unwrap().to_uppercase();

            match op_code.as_str() {
                "NOP" => Ok(Instruction::Nop),
                "MOV" => Ok(Instruction::Mov {
                    src: splitted
                        .next()
                        .ok_or(ParseError::InvalidArgumentNumber)?
                        .parse()?,
                    dst: splitted
                        .next()
                        .ok_or(ParseError::InvalidArgumentNumber)?
                        .parse()?,
                }),
                "SWP" => Ok(Instruction::Swp),
                "SAV" => Ok(Instruction::Sav),
                "ADD" => Ok(Instruction::Add(splitted
                    .next()
                    .ok_or(ParseError::InvalidArgumentNumber)?
                    .parse()?)),
                "SUB" => Ok(Instruction::Sub(splitted
                    .next()
                    .ok_or(ParseError::InvalidArgumentNumber)?
                    .parse()?)),
                "NEG" => Ok(Instruction::Neg),
                "JMP" => Ok(Instruction::Jmp(
                    splitted
                        .next()
                        .ok_or(ParseError::InvalidArgumentNumber)?
                        .to_owned(),
                )),
                "JEZ" => Ok(Instruction::Jez(
                    splitted
                        .next()
                        .ok_or(ParseError::InvalidArgumentNumber)?
                        .to_owned(),
                )),
                "JNZ" => Ok(Instruction::Jnz(
                    splitted
                        .next()
                        .ok_or(ParseError::InvalidArgumentNumber)?
                        .to_owned(),
                )),
                "JGZ" => Ok(Instruction::Jgz(
                    splitted
                        .next()
                        .ok_or(ParseError::InvalidArgumentNumber)?
                        .to_owned(),
                )),
                "JLZ" => Ok(Instruction::Jlz(
                    splitted
                        .next()
                        .ok_or(ParseError::InvalidArgumentNumber)?
                        .to_owned(),
                )),
                "JRO" => Ok(Instruction::Jro(splitted
                    .next()
                    .ok_or(ParseError::InvalidArgumentNumber)?
                    .parse()?)),
                _ => Err(ParseError::InvalidIdentifier),
            }.and_then(|i| {
                if splitted.next().is_some() {
                    Err(ParseError::InvalidArgumentNumber)
                } else {
                    Ok(i)
                }
            })
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

impl FromStr for Source {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        i16::from_str(s)
            .map(Source::Value)
            .map_err(|_| ParseError::InvalidIdentifier)
            .or_else(|_| Destination::from_str(s).map(Source::Destination))
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

impl FromStr for Destination {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Destination::*;

        let s = s.to_uppercase();
        match s.as_str() {
            "ACC" => Ok(Acc),
            "NIL" => Ok(Nil),
            s => self::Port::from_str(s).map(Port),
        }
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

impl FromStr for Port {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Port::*;

        let s = s.to_uppercase();
        match s.as_str() {
            "ANY" => Ok(Any),
            "DOWN" => Ok(Down),
            "LAST" => Ok(Last),
            "LEFT" => Ok(Left),
            "UP" => Ok(Up),
            "RIGHT" => Ok(Right),
            _ => Err(ParseError::InvalidIdentifier),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        use self::Instruction::*;

        assert_eq!(
            Ok(Label {
                name: "FOO".to_string(),
                instruction: None
            }),
            "FOO: ".parse()
        );
        assert_eq!(
            Ok(Label {
                name: "FOO".to_string(),
                instruction: Some(Box::new(Nop))
            }),
            "FOO: NOP".parse()
        );
        assert_eq!(Ok(Nop), "NOP".parse());
        assert_eq!(
            Ok(Mov {
                src: Source::Value(42),
                dst: Destination::Nil
            }),
            "MOV 42 NIL".parse()
        );
        assert_eq!(Ok(Add(Source::Value(42))), "ADD   42".parse());
        assert_eq!(Ok(Sub(Source::Value(21))), " SUB   21".parse());
        assert_eq!(Ok(Neg), " NEG ".parse());
        assert_eq!(Ok(Jmp("FOO".to_owned())), "JMP FOO".parse());
        assert_eq!(Ok(Jez("BAR".to_owned())), "jez BAR".parse());
        assert_eq!(Ok(Jnz("FOO".to_owned())), "JNZ FOO".parse());
        assert_eq!(Ok(Jgz("BAR".to_owned())), "jgZ BAR".parse());
        assert_eq!(Ok(Jlz("FOO".to_owned())), "JLZ FOO".parse());

        assert_eq!(
            Err(ParseError::InvalidIdentifier),
            "FOO".parse::<Instruction>()
        );
        assert_eq!(
            Err(ParseError::InvalidIdentifier),
            "FOO BAR".parse::<Instruction>()
        );
    }

    #[test]
    fn test_parse_source() {
        assert_eq!(Ok(Source::Value(42)), "42".parse());
        assert_eq!(Ok(Source::Destination(Destination::Acc)), "acc".parse());
    }

    #[test]
    fn test_parse_destination() {
        assert_eq!(Ok(Destination::Acc), "ACC".parse());
        assert_eq!(Ok(Destination::Nil), "NiL".parse());
        assert_eq!(Ok(Destination::Port(Port::Last)), "LAST".parse());
    }

    #[test]
    fn test_parse_port() {
        assert_eq!(Ok(Port::Up), "UP".parse());
        assert_eq!(Ok(Port::Down), "Down".parse());
        assert_eq!(Ok(Port::Right), "RIGHT".parse());
        assert_eq!(Ok(Port::Left), "left".parse());
        assert_eq!(Ok(Port::Any), "any".parse());
        assert_eq!(Ok(Port::Last), "LAST".parse());
    }
}
