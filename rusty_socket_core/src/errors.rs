use std::fmt;

pub type RS_Result<T> = Result<T, RS_Error>;

#[derive(Debug)]
pub enum RS_Error {
    ProtocolError,
    MethodNotAllowed,
    BadRequest,
    UnprocessableContent,
    UpgradeRequired,
}

impl fmt::Display for RS_Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RS_Error::ProtocolError => write!(f, "Protocol Error"),
            RS_Error::MethodNotAllowed => write!(f, "Method Not Allowed"),
            RS_Error::BadRequest => write!(f, "Bad Request"),
            RS_Error::UnprocessableContent => write!(f, "Unprocessable Content"),
            RS_Error::UpgradeRequired => write!(f, "Upgrade Required")
        }
    }
}

impl std::error::Error for RS_Error {}