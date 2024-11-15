use core::ffi::FromBytesUntilNulError;

use std::{
    fmt, io,
    num::ParseIntError,
    process::{ExitStatus, Output},
    str::Utf8Error,
};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Underlying I/O error
    IO(io::Error),

    /// Underlying UTF error
    Encoding(Utf8Error),

    /// C-String error
    CString(FromBytesUntilNulError),

    /// Cannot parse package
    PackageFormat(String),

    /// Package is missing
    PackageNotFound(String),

    /// Subprocess invocation error
    CalledProcess {
        status: ExitStatus,
        error: Option<String>,
    },

    /// Parse integer error
    Integer(ParseIntError),

    /// Missing field
    MissingField(String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<Utf8Error> for Error {
    fn from(e: Utf8Error) -> Self {
        Self::Encoding(e)
    }
}

impl From<FromBytesUntilNulError> for Error {
    fn from(e: FromBytesUntilNulError) -> Self {
        Self::CString(e)
    }
}

impl From<Output> for Error {
    fn from(o: Output) -> Self {
        assert!(!o.status.success());

        let msg = String::from_utf8(o.stderr)
            .ok()
            .and_then(|s| if s.is_empty() { None } else { Some(s) })
            .map(|s| String::from(s.trim()));

        Self::CalledProcess {
            status: o.status,
            error: msg,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::Integer(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(ref e) => fmt::Display::fmt(e, f),
            Self::Encoding(ref e) => fmt::Display::fmt(e, f),
            Self::CString(ref e) => fmt::Display::fmt(e, f),
            Self::PackageFormat(ref pf) => write!(f, "Cannot parse package from {:?}", pf),
            Self::PackageNotFound(ref name) => write!(f, "Cannot find package {name:?}"),
            Self::CalledProcess {
                ref status,
                ref error,
            } => {
                if let Some(code) = status.code() {
                    if let Some(msg) = error {
                        write!(f, "Process exited with code {}: {}", code, msg)
                    } else {
                        write!(f, "Process exited with code {}", code)
                    }
                } else if let Some(msg) = error {
                    write!(f, "Process exited with error: {}", msg)
                } else {
                    f.write_str("Process exited with error")
                }
            }
            Self::Integer(ref e) => fmt::Display::fmt(e, f),
            Self::MissingField(ref fieldname) => write!(f, "Missing field {}", fieldname),
        }
    }
}
