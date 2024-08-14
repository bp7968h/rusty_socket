use std::fmt;

pub type RsResult<T> = Result<T, RsError>;

#[derive(Debug)]
pub enum RsError {
    ProtocolError,
    MethodNotAllowed,
    BadRequest,
    UnprocessableContent,
    UpgradeRequired,
    IncompleteData,
    FragmentationNotSupported,
    InvalidOpCode,
}

impl fmt::Display for RsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RsError::ProtocolError => write!(f, "Protocol Error"),
            RsError::MethodNotAllowed => write!(f, "Method Not Allowed"),
            RsError::BadRequest => write!(f, "Bad Request"),
            RsError::UnprocessableContent => write!(f, "Unprocessable Content"),
            RsError::UpgradeRequired => write!(f, "Upgrade Required"),
            RsError::IncompleteData => write!(f, "Insufficient Data"),
            RsError::FragmentationNotSupported => write!(f, "Fragmentation Not yet Supported"),
            RsError::InvalidOpCode => write!(f, "Invalid Opcode"),
        }
    }
}

impl std::error::Error for RsError {}
