use std::collections::HashMap;

pub struct RequestLine {
    method: String,
    resource : String,
    headers: HashMap<String, String>
}

impl RequestLine {
    fn new() -> Self {
        RequestLine {
            method: String::new(),
            resource: String::new(),
            headers: HashMap::new(),
        }
    }


}