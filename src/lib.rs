//! Emulate Nintendo Switch controllers over Bluetooth.

use log::LevelFilter;
use std::fmt::{self, Display, Formatter};

mod bluetooth;
mod logger;

use logger::Logger;

/// Enumeration of error kinds.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorKind {
    /// Represents the Bluetooth error.
    Bluetooth,
    /// Represents the protocol error.
    Protocol,
    /// Represents the IO error.
    Io,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ErrorKind::Bluetooth => write!(f, "bluetooth"),
            ErrorKind::Protocol => write!(f, "protocol"),
            ErrorKind::Io => write!(f, "io"),
        }
    }
}

/// Represents an error.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Error {
    /// Represents the error kind.
    pub kind: ErrorKind,
    /// Represents the detailed message.
    pub message: String,
}

impl Error {
    /// Creates a `Error`.
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Error { kind, message }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            kind,
            message: String::new(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "{}", &self.kind)
        } else {
            write!(f, "{}: {}", &self.kind, &self.message)
        }
    }
}

/// Represents an result.
pub type Result<T> = std::result::Result<T, Error>;

// Initializes the logger and set its level.
pub fn set_logger(verbose: usize) {
    let level = match verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    Logger::init(level);
}
