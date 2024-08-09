use crate::ScError;
use crate::WebSocketUrl;

use std::net::{TcpStream};

pub struct SocketClient {}

impl SocketClient{
    pub fn build(url: &str) -> Result<Self, ScError> {
        let result_url = WebSocketUrl::from_url(url);

        match result_url {
            Ok(parsed_url) => {
                match TcpStream::connect(&parsed_url){
                    Ok(stream) => {
                        todo!()
                    },
                    Err(e) => Err(ScError::from(e)),
                }
            },
            Err(e) => Err(e),
        }
    }
}