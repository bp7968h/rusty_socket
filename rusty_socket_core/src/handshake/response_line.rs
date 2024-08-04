use std::collections::HashMap;

use cryptography::SHA1;
use base64::encode;

const WS_GUID: &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub struct ResponseLine {
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: Option<HashMap<String, String>>
}

impl ResponseLine {
    pub fn err_build(status_code: u16, reason_phrase: &str) -> Self {
        ResponseLine {
            status_code,
            reason_phrase: reason_phrase.to_string(),
            headers: None
        }
    }

    pub fn build(request_key: &str) -> Self {
        let mut response_headers: HashMap<String, String> = HashMap::new();
        let accpet_key = Self::generate_websocket_accept_key(request_key);

        response_headers.insert("Upgrade".to_string(), "websocket".to_string());
        response_headers.insert("Connection".to_string(), "Upgrade".to_string());
        response_headers.insert("Sec-WebSocket-Accept".to_string(), accpet_key);

        ResponseLine {
            status_code: 101,
            reason_phrase: String::from("Switching Protocols"),
            headers: Some(response_headers)
        }

    }

    fn generate_websocket_accept_key(key: &str) -> String {
        let mut hasher = SHA1::new();
        let mut combined_key = key.to_string();
        combined_key.push_str(WS_GUID);
        
        let sha1_hash = hasher.hash(&combined_key);
        
        encode(&sha1_hash)
    }
}