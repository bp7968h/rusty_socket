use crate::ScError;

#[derive(Debug)]
pub struct WebSocketUrl {
    scheme: String,
    host: String,
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
}

impl WebSocketUrl {
    fn new() -> Self {
        WebSocketUrl {
            scheme: String::new(),
            host: String::new(),
            path: None,
            query: None,
            fragment: None,
        }
    }

    pub fn from_url(url: &str) -> Result<Self, ScError>{
        todo!()
    }
}