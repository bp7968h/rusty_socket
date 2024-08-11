use crate::ScError;

use std::collections::HashMap;

use cryptography::SHA1;
use base64;

const WS_GUID: &'static str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";


pub fn verify_status_line(status_line: &str) -> Result<(), ScError> {
    let resp_line: Vec<&str> = status_line.splitn(3, ' ').collect();
    if resp_line.len() != 3 {
        return Err(ScError::InvalidHttpResponse);
    }

    match resp_line[0].split_once('/') {
        Some((protocol, version)) => {
            if protocol.to_ascii_lowercase() != "http" {
                return Err(ScError::InvalidHttpResponse);
            }
            match version.parse::<f32>() {
                Ok(ver_num) => {
                    if ver_num < 1.1 {
                        return Err(ScError::LowerHttpVersion);
                    }
                },
                Err(_) => {
                    return Err(ScError::InvalidHttpResponse);
                },
            }
        },
        None => {
            return Err(ScError::InvalidHttpResponse);
        }
    }

    match resp_line[1].parse::<u16>() {
        Ok(status_code) => {
            if status_code != 101 {
                return Err(ScError::InvalidStatusCode);
            }
        },
        Err(_) => {
            return Err(ScError::InvalidHttpResponse);
        },
    }

    if resp_line[2].to_ascii_lowercase() != "switching protocols" {
        return Err(ScError::InvalidHttpResponse);
    }

    return Ok(())
}

pub fn validate_headers(resp_headers: &HashMap<String, String>, client_key: &str) -> Result<(), ScError>{
    match resp_headers.get("upgrade") {
        Some(upgrade_value) => {
            if upgrade_value.to_ascii_lowercase() != "websocket" {
                return Err(ScError::InvalidHandshakeHeader);
            }
        },
        None => return Err(ScError::InvalidHandshakeHeader),
    }

    match resp_headers.get("connection") {
        Some(conn_value) => {
            if conn_value.to_ascii_lowercase() != "upgrade" {
                return Err(ScError::InvalidHandshakeHeader);
            }
        },
        None => return Err(ScError::InvalidHandshakeHeader),
    }

    match resp_headers.get("sec-websocket-accept") {
        Some(accept_key) => {
            let mut hasher = SHA1::new();
            let mut client_key = client_key.to_string();
            client_key.push_str(WS_GUID);
            
            let sha1_hash = hasher.hash(&client_key);
            let generated_accept_key = base64::encode(&sha1_hash);

            if generated_accept_key != *accept_key {
                return Err(ScError::InvalidHandshakeHeader);
            }

        },
        None => return Err(ScError::InvalidHandshakeHeader)
    }

    Ok(())
}