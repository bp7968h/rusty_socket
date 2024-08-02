use std::collections::HashMap;

pub struct ResponseLine {
    status_code: u16,
    reason_phrase: String,
    headers: HashMap<String, String>
}
