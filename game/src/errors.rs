#[derive(Clone, Debug, Fail, Eq, PartialEq)]
pub enum ParseError {
    #[fail(display = "Invalid argument number")]
    InvalidArgumentNumber,
    #[fail(display = "Invalid identifier")]
    InvalidIdentifier,
    #[fail(display = "Invalid label")]
    InvalidLabel,
}

#[derive(Clone, Debug, Fail, Eq, PartialEq)]
pub enum RuntimeError {
    #[fail(display = "")]
    UnknownLabelName,
}
