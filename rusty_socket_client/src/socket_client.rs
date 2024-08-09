use crate::ScError;
use crate::WebSocketUrl;

pub struct SocketClient {}

impl SocketClient{
    pub fn build(url: &str) -> Result<Self, ScError> {
        let result_url = WebSocketUrl::from_url(url);

        todo!()
    }
}