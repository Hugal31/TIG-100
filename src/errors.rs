use std::fmt;
use std::io;

use failure::{Backtrace, Context, Fail};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Display(DisplayError),
}

impl<T: Into<DisplayError>> From<T> for Error {
    fn from(error: T) -> Self {
        Error::Display(error.into())
    }
}

pub type IoError = io::Error;

#[derive(Debug)]
pub struct DisplayError {
    inner: Context<DisplayErrorKind>,
}

impl Fail for DisplayError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<DisplayErrorKind> for DisplayError {
    fn from(error: DisplayErrorKind) -> Self {
        DisplayError {
            inner: Context::new(error),
        }
    }
}

impl From<Context<DisplayErrorKind>> for DisplayError {
    fn from(ctx: Context<DisplayErrorKind>) -> Self {
        DisplayError { inner: ctx }
    }
}

#[derive(Copy, Clone, Debug, Fail, Eq, PartialEq)]
pub enum DisplayErrorKind {
    /// Any IO error
    #[fail(display = "An IO error occured")]
    IoError,
    /// The screen is too small, and require .0 in width and .1 in height.
    #[fail(display = "The screen must be at least {} by {}", _0, _1)]
    ScreenToSmall(u16, u16),
}
