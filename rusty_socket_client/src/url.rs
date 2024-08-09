use crate::ScError;

#[derive(Debug)]
pub struct WebSocketUrl {
    pub scheme: String,
    pub host: String,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
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
        let mut new_wsu = Self::new();
        
        if !url.contains("://"){
            return Err(ScError::InvalidUrl);
        }
        
        let parts : Vec<&str> = url.split("://").collect();
        new_wsu.scheme = parts[0].to_string();
            
        if let Some((host, path_query)) = parts[1].split_once('/') {
            if host.is_empty() {
                return Err(ScError::InvalidUrl);
            }
            
            new_wsu.host = host.to_string();
            
            if path_query.is_empty(){
                return Ok(new_wsu);
            }
            
            if let Some((path, query_fragment)) = path_query.split_once('?'){
                new_wsu.path = Some(path.to_string());

                if query_fragment.is_empty() {
                    return Ok(new_wsu);
                }
                
                match query_fragment.split_once('#') {
                    Some((query, fragment)) => {
                        if query.is_empty(){
                            return Ok(new_wsu);
                        }

                        new_wsu.query = Some(query.to_string());
                    
                        if fragment.is_empty(){
                            return Ok(new_wsu);
                        }
                        new_wsu.fragment = Some(fragment.to_string());
                    },
                    None => new_wsu.query = Some(query_fragment.to_string()),
                }
                
            } else {
                new_wsu.path = Some(path_query.to_string());
            }
        } else {
            new_wsu.host = parts[1].to_string();
        }
        
        Ok(new_wsu)
    }
}