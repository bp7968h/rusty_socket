use crate::ScError;
use crate::WebSocketUrl;
use crate::utils;

use base64::encode;
use rand::RngCore;

use std::io::{Write, Read};
use std::net::{Shutdown, TcpStream};
use std::collections::HashMap;
use rusty_socket_core::{DataFrame, OpCode};

pub struct SocketClient {
    pub stream: TcpStream,
}

impl SocketClient{
    pub fn build(url: &str) -> Result<Self, ScError> {
        let result_url = WebSocketUrl::from_url(url);

        match result_url {
            Ok(parsed_url) => {
                match TcpStream::connect(&parsed_url){
                    Ok(stream) => {
                        let frame_stream = Self::perform_handshake(stream, parsed_url)?;

                        Ok(SocketClient{stream: frame_stream})
                    },
                    Err(e) => Err(ScError::from(e)),
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn send(&mut self, message: &str) -> Result<(), ScError> {
        if message.len() == 0 {
            return Err(ScError::DataFrameError);
        }
        match DataFrame::from_data(message, OpCode::Text, true) {
            Some(frame) => {
                self.stream.write_all(&Vec::from(frame)).map_err(ScError::from)?;
                self.stream.flush().map_err(ScError::from)?;

                Ok(())
            },
            None => Err(ScError::DataFrameError),
        }
    }

    pub fn close(&mut self) -> Result<(), ScError>{
        //TODO send close frame
        self.stream.shutdown(Shutdown::Both).map_err(ScError::from)?;

        Ok(())
    }

    fn perform_handshake(mut stream: TcpStream, url: WebSocketUrl) -> Result<TcpStream, ScError>{
        let resource_name = url.resource_name();
        let host = match url.host.find(':') {
            Some(idx) => {
                url.host[..idx].to_string()
            },
            None => {
                url.host
            }
        };

        let websocket_key = Self::generate_key();

        let websocket_request = format!(
            "GET {} HTTP/1.1\r\n\
            Host: {}\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Key: {}\r\n\
            Sec-WebSocket-Version: 13\r\n\r\n",
            resource_name, host, websocket_key
        );

        stream.write_all(websocket_request.as_bytes()).map_err(ScError::from)?;
        stream.flush().map_err(ScError::from)?;

        Self::verify_handshake_response(&websocket_key, &mut stream)?;

        Ok(stream)
    }

    fn generate_key() -> String {
        let mut nonce: [u8; 16] = [0; 16];
        rand::thread_rng().fill_bytes(&mut nonce);

        encode(&nonce)
    }

    fn verify_handshake_response(key: &str, stream: &mut TcpStream) -> Result<(), ScError> {
        let mut buffer = [0u8; 512];
        loop {
            match stream.read(&mut buffer) {
                Ok(n) if n == 0 => {
                    return Err(ScError::ServerClosed);
                },
                Ok(n) => {
                    let received_data = String::from_utf8_lossy(&buffer[..n]);
                    let mut lines = received_data.lines();

                    if let Some(status_line) = lines.next() {
                        let _ = utils::verify_status_line(&status_line)?;
                    }

                    let mut resp_headers: HashMap<String, String> = HashMap::new();
                    for line in lines {
                        if line.is_empty(){
                            break;
                        }

                        if let Some((key, value)) = line.split_once(": "){
                            let l_key = key.to_ascii_lowercase();
                            resp_headers.insert(l_key, value.to_string());
                        }
                    }

                    let _ = utils::validate_headers(&resp_headers, &key)?;

                    return Ok(())
                },
                Err(e) => {
                    return Err(ScError::from(e));
                }
            }
        }
    }
}