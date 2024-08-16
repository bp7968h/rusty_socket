use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ScError {
    InvalidUrl,
    IoError(io::Error),
    ServerClosed,
    InvalidHttpResponse,
    InvalidStatusCode,
    LowerHttpVersion,
    InvalidHandshakeHeader,
    DataFrameError,
}

impl PartialEq for ScError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ScError::InvalidUrl, ScError::InvalidUrl) => true,
            (ScError::ServerClosed, ScError::ServerClosed )=> true,
            (ScError::InvalidStatusCode, ScError::InvalidStatusCode )=> true,
            (ScError::LowerHttpVersion, ScError::LowerHttpVersion )=> true,
            (ScError::InvalidHttpResponse, ScError::InvalidHttpResponse )=> true,
            (ScError::InvalidHandshakeHeader, ScError::InvalidHandshakeHeader )=> true,
            (ScError::DataFrameError, ScError::DataFrameError )=> true,
            (ScError::IoError(e1), ScError::IoError(e2)) => e1.kind() == e2.kind(),
            _ => false,
        }
    }
}

impl fmt::Display for ScError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidUrl => write!(f, "Invalid websocket url received."),
            Self::IoError(e) => write!(f, "I/O error: {}", e),
            Self::ServerClosed => write!(f, "Connection closed by server."),
            Self::LowerHttpVersion => write!(f, "Unsupported http Version, is less than 1.1"),
            Self::InvalidStatusCode => write!(f, "Status Code is not 101"),
            Self::InvalidHttpResponse => write!(f, "Invalid http response line"),
            Self::InvalidHandshakeHeader => write!(f, "Handshake response header invalid."),
            Self::DataFrameError => write!(f, "Failed to create dataframe"),
        }
    }
}

impl From<io::Error> for ScError {
    fn from(error: io::Error) -> Self {
        ScError::IoError(error)
    }
}

impl std::error::Error for ScError {}