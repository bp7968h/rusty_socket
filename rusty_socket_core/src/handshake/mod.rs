pub mod request_line;
pub mod response_line;
pub mod connection_status;

pub use request_line::RequestLine;
pub use response_line::ResponseLine;
pub use connection_status::ConnectionStatus;

pub struct HandShake {
    request: Option<RequestLine>,
    response: ResponseLine,
    state: ConnectionStatus
}

impl HandShake{
    pub fn build(mut full_request: &str) -> Self {
        if let Some(request) = RequestLine::from_request(full_request.lines()){
            if let Some(web_socket_key) = request.headers.get("sec-websocket-key") {
                let response = ResponseLine::build(web_socket_key);
                HandShake {
                    request: Some(request),
                    response: response,
                    state: ConnectionStatus::Connecting
                }
            } else {
                let response = ResponseLine::err_build(400, "Bad Request: Missing Sec-WebSocket-Key");
                HandShake {
                    request: None,
                    response,
                    state: ConnectionStatus::Closing,
                }
            }
        } else {
            let response = ResponseLine::err_build(404, "Method Not Allowed");
            HandShake {
                request : None,
                response: response,
                state: ConnectionStatus::Closing,
            }
        }
    }
}