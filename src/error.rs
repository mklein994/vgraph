use std::error;
use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub enum Error {
    CharParse,
    Getopts(getopts::Fail),
    Io(io::Error),
    OutOfBounds,
    ParseFloat(num::ParseFloatError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CharParse => write!(f, "Could not parse amount as a char"),
            Error::Getopts(err) => err.fmt(f),
            Error::Io(err) => err.fmt(f),
            Error::OutOfBounds => write!(f, "When given a single argument, it must be a decimal between 0.0 and 1.0 (inclusive)"),
            Error::ParseFloat(err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<getopts::Fail> for Error {
    fn from(err: getopts::Fail) -> Self {
        Error::Getopts(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Self {
        Error::ParseFloat(err)
    }
}
