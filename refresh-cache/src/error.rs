use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    MissingField(&'static str),
    ParseIntError(ParseIntError),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}
