use std::{fmt, io, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    MissingField(&'static str),
    ParseInt(ParseIntError),
    Io(io::Error),
    InvalidTimeSuffix(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingField(s) => write!(f, "Field {} is missing", *s),
            Error::ParseInt(ref e) => fmt::Display::fmt(e, f),
            Error::Io(ref e) => fmt::Display::fmt(e, f),
            Error::InvalidTimeSuffix(ref s) => write!(f, "Invalid time suffix {:?}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseInt(e)
    }
}
