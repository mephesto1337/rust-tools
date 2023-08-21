use std::{fmt, io, str::Utf8Error};

pub type Result<T> = std::result::Result<T, CodecError>;

/// Errors for this crate
#[derive(Debug)]
pub enum CodecError {
    /// Underlying I/O error
    IO(io::Error),

    /// Base64 decode error
    Base64(base64::DecodeError),

    /// Invalid digit
    InvalidHexDigit(u8),

    /// Invalid UTF-8 sequence
    UTF8(Utf8Error),

    /// Encounter non-ascii character
    NonAsciiChar(u8),

    /// Cannot find any codec that can decode Input
    NoCodecAvailable,
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodecError::IO(ref e) => fmt::Display::fmt(e, f),
            CodecError::Base64(ref e) => fmt::Display::fmt(e, f),
            CodecError::InvalidHexDigit(ref b) => {
                write!(f, "0x{:02x} is not a valid hexadecimal digit", b)
            }
            CodecError::UTF8(ref e) => fmt::Display::fmt(e, f),
            CodecError::NonAsciiChar(ref b) => {
                write!(f, "Encounter non-ascii character 0x{:02x}", b)
            }
            CodecError::NoCodecAvailable => f.write_str("Cannot find a suitable codec"),
        }
    }
}

impl std::error::Error for CodecError {}

impl From<io::Error> for CodecError {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<base64::DecodeError> for CodecError {
    fn from(e: base64::DecodeError) -> Self {
        Self::Base64(e)
    }
}

impl From<Utf8Error> for CodecError {
    fn from(e: Utf8Error) -> Self {
        Self::UTF8(e)
    }
}
