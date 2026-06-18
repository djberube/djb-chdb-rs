//! Error type for chdb operations.

use std::fmt;

/// Errors returned by [`Connection`](crate::Connection) operations.
#[derive(Debug)]
pub enum Error {
    /// `chdb_connect` returned null — the embedded engine could not be started
    /// (for example, an invalid `--path` or a second connection in a process
    /// that already has one open).
    ConnectFailed,

    /// A connect argument or query string contained an interior NUL byte and
    /// could not be passed to the C API.
    NulByte(std::ffi::NulError),

    /// `chdb_query` returned null without an error message.
    NullResult,

    /// The query executed but chdb reported an error.
    Query(String),

    /// The result buffer was not valid UTF-8 (only returned by
    /// [`QueryResult::data_utf8`](crate::QueryResult::data_utf8)).
    Utf8(std::string::FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConnectFailed => write!(f, "failed to open chdb connection"),
            Error::NulByte(e) => write!(f, "argument contained an interior NUL byte: {e}"),
            Error::NullResult => write!(f, "chdb_query returned a null result"),
            Error::Query(msg) => write!(f, "chdb query error: {msg}"),
            Error::Utf8(e) => write!(f, "result was not valid UTF-8: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::NulByte(e) => Some(e),
            Error::Utf8(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(e: std::ffi::NulError) -> Self {
        Error::NulByte(e)
    }
}

/// Convenience alias for results from this crate.
pub type Result<T> = std::result::Result<T, Error>;
