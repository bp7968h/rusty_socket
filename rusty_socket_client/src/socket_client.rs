use crate::ScError;
use crate::WebSocketUrl;

use base64::encode;
use rand::RngCore;

use std::io::Write;
use std::net::{TcpStream};

pub struct SocketClient {
    stream: TcpStream,
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
            "GET {} HTTP/1.1\r\n
            Host: {}\r\n
            Upgrade: websocket\r\n
            Connection: Upgrade\r\n
            Sec-WebSocket-Key: {}\r\n
            Sec-WebSocket-Version: 13\r\n\r\n", 
            resource_name, host, websocket_key
        );
        
        stream.write_all(websocket_request.as_bytes()).map_err(ScError::from)?;
        stream.flush().map_err(ScError::from)?;

        Ok(stream)
    }

    fn generate_key() -> String {
        let mut nonce: [u8; 16] = [0; 16];
        rand::thread_rng().fill_bytes(&mut nonce);

        encode(&nonce)
    }
}