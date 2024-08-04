use std::collections::HashMap;
use std::str::Lines;

pub struct RequestLine {
    pub resource : String,
    pub headers: HashMap<String, String>
}

impl RequestLine {
    pub fn new() -> Self {
        RequestLine {
            resource: String::new(),
            headers: HashMap::new(),
        }
    }

    pub fn from_request(mut full_request: Lines) -> Option<Self> {
        let mut resource = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();

        if let Some(first_line) = full_request.next() {
            let req_line: Vec<&str> = first_line.split_whitespace().collect();
            if req_line.len() != 3 && req_line[0] != "GET" {
                return None;
            }
            resource = req_line[1].to_string();
        }

        for line in full_request {
            if line.is_empty(){
                break;
            }

            if let Some((key, value)) = line.split_once(": "){
                let l_key = key.to_ascii_lowercase();
                headers.insert(l_key, value.to_string());
            }
        }

        match Self::validate_headers(&headers) {
            Ok(_) => Some(RequestLine {
                resource,
                headers,
            }),
            Err(e) => None
        }
    }

    fn validate_headers(headers: &HashMap<String,String>) -> Result<(), &'static str> {
        //validate version
        if let Some(version) = headers.get("sec-websocket-version") {
            match version.parse::<u8>() {
                Ok(v) => {
                    if v != 13 {
                        return Err("Invalid WebSocket Version");
                    } 
                },
                Err(_) => return Err("Invalid WebSocket Version")
            }
        } else {
            return Err("Missing WebSocket Version");
        }

        //Validate Upgrade
        if let Some(upgrade_value) = headers.get("upgrade") {
            if upgrade_value.to_ascii_lowercase() != "websocket" {
                return Err("Invalid Upgrade Value");
            }
        } else {
            return Err("Missing Upgrade Header");
        }
        //validate Connection
        if let Some(connection_value) = headers.get("connection") {
            if connection_value.to_ascii_lowercase() != "upgrade" {
                return Err("Invalid Connection Value");
            }
        } else {
            return Err("Missing Connection Header");
        }

        Ok(())
    }


}