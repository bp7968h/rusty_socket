use std::{fmt, io};

#[derive(Debug)]
pub enum SsError {
    InvalidBindAddress,
    IoError(io::Error),
}

impl fmt::Display for SsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBindAddress => write!(f, "Cannot bind to provided address"),
            Self::IoError(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for SsError {
    fn from(error: io::Error) -> Self {
        SsError::IoError(error)
    }
}

impl std::error::Error for SsError {}

