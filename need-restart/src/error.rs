use std::{ffi, fmt, io};

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

    /// getpwname_r error
    GetPwname(io::Error),

    /// Invalid C-String
    InvalidCString(ffi::NulError),

    /// Unknown username
    UnknownUser(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<ffi::NulError> for Error {
    fn from(e: ffi::NulError) -> Self {
        Self::InvalidCString(e)
    }
}

impl Error {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::IO(e) if e.kind() == io::ErrorKind::NotFound)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonExistantProcess(pid) => write!(f, "Non existing pid {}", pid),
            Self::NoExecutableFile(pid) => write!(f, "No executable file for pid {}", pid),
            Self::IO(e) => fmt::Display::fmt(e, f),
            Self::InvalidArgument(arg) => write!(f, "Invalid command line argument {:?}", arg),
            Self::GetPwuid(e) => write!(f, "Error with getpwuid_r: {e}"),
            Self::GetPwname(e) => write!(f, "Error with getpwname_r: {e}"),
            Self::InvalidCString(e) => write!(f, "Invalid C String: {e}"),
            Self::UnknownUser(name) => write!(f, "Unknown user {name:?}"),
        }
    }
}
