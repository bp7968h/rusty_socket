use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ScError {
    InvalidUrl,
    IoError(io::Error),
}

impl PartialEq for ScError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScError::InvalidUrl, ScError::InvalidUrl) => true,
            (ScError::IoError(e1), ScError::IoError(e2)) => e1.kind() == e2.kind(),
            _ => false,
        }
    }
}

impl fmt::Display for ScError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidUrl => write!(f, "Invalid websocket url reveived."),
            Self::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl From<io::Error> for ScError {
    fn from(error: io::Error) -> Self {
        ScError::IoError(error)
    }
}

impl std::error::Error for ScError {}