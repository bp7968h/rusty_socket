use std::collections::HashMap;

pub struct RequestLine {
    method: String,
    resource : String,
    headers: HashMap<String, String>
}