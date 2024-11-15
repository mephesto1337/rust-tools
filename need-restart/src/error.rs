use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Non-existance process ID
    NonExistantProcess(u64),

    /// No executable found
    NoExecutableFile(u64),

    /// Standard I/O error
    IO(io::Error),

    /// Invalid CLI argument
    InvalidArgument(String),

    /// getpwuid_r error
    GetPwuid(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::IO(ref e) if e.kind() == io::ErrorKind::NotFound)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonExistantProcess(pid) => write!(f, "Non existing pid {}", pid),
            Self::NoExecutableFile(pid) => write!(f, "No executable file for pid {}", pid),
            Self::IO(ref e) => fmt::Display::fmt(e, f),
            Self::InvalidArgument(ref arg) => write!(f, "Invalid command line argument {:?}", arg),
            Self::GetPwuid(ref e) => write!(f, "Error with getpwuid_r: {e}"),
        }
    }
}
