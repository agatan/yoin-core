use std::io;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    InvalidMorph,
    IO(io::Error),
    InvalidEncode,
    InvalidMatrix,
    InvalidChardef,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidMorph => write!(f, "invalid morphemes"),
            Error::InvalidEncode => write!(f, "invalid file encoding"),
            Error::InvalidMatrix => write!(f, "invalid matrix"),
            Error::InvalidChardef => write!(f, "invalid chardef"),
            Error::IO(ref err) => write!(f, "{}", err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "dictionary build error"
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            Error::IO(ref err) => Some(err),
            _ => None,
        }
    }
}
