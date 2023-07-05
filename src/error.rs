// error handling
use std::num::ParseIntError;
use std::{error::Error as StdError, fmt, result};

/// A type alias for `Result<T, refer::Error>`.
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

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::new(ErrorKind::ParseInt(err))
    }
}

/// The specific type of error that can occur.
#[derive(Debug)]
pub enum ErrorKind {
    ParseError(String),
    FromStr(String),
    ParseInt(ParseIntError),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self.0 {
            ErrorKind::ParseError(err) => err.fmt(f),
            ErrorKind::FromStr(err) => err.fmt(f),
            ErrorKind::ParseInt(ref err) => err.fmt(f),
        }
    }
}
