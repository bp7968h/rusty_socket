use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ScError {
    InvalidUrl,
}

impl fmt::Display for ScError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidUrl => write!(f, "Invalid websocket url reveived."),
        }
    }
}

impl std::error::Error for ScError {}