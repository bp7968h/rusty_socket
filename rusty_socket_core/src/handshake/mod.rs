pub mod request_line;
pub mod response_line;
pub mod connection_status;

pub use request_line::RequestLine;
pub use response_line::ResponseLine;
pub use connection_status::ConnectionStatus;

pub struct HandShake {
    request: RequestLine,
    response: ResponseLine,
    state: ConnectionStatus
}