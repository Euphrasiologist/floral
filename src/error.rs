// error handling
use std::{error::Error as StdError, fmt, result};

use pico_args::Error as PicoError;

/// A type alias for `Result<T, floral::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error that can happen.
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// A crate private constructor for `Error`.
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error(Box::new(kind))
    }

    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

impl From<PicoError> for Error {
    fn from(err: PicoError) -> Self {
        Error::new(ErrorKind::Cli(err))
    }
}

/// The specific type of error that can occur.
#[derive(Debug)]
pub enum ErrorKind {
    ParseError(String),
    FromStr(String),
    ParseInt(String),
    CSVParseError(String),
    Cli(PicoError),
    GenericCli(String),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.0 {
            ErrorKind::ParseError(err) => err.fmt(f),
            ErrorKind::FromStr(err) => err.fmt(f),
            ErrorKind::ParseInt(ref err) => err.fmt(f),
            ErrorKind::CSVParseError(err) => err.fmt(f),
            ErrorKind::Cli(err) => err.fmt(f),
            ErrorKind::GenericCli(err) => err.fmt(f),
        }
    }
}
