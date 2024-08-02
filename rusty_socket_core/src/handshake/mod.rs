pub mod request_line;
pub mod response_line;
pub mod connection_status;

use request_line::RequestLine;
use response_line::ResponseLine;
use connection_status::ConnectionStatus;

pub struct HandShake {
    request: RequestLine,
    response: ResponseLine,
    state: ConnectionStatus
}