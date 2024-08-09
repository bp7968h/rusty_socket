use crate::ScError;
use std::net::{SocketAddr, ToSocketAddrs};
use std::io;

#[derive(Debug)]
pub struct WebSocketUrl {
    pub scheme: String,
    pub host: String,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

impl ToSocketAddrs for WebSocketUrl {
    type Iter = std::vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        let port = match self.port() {
            Ok(port) => port,
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid port or scheme"));
            }
        };

        let host = if let Some(idx) = self.host.find(':') {
            &self.host[..idx]
        } else {
            &self.host
        };

        let addr_string = format!("{}:{}", host, port);
        
        addr_string.to_socket_addrs()
    }
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

    fn port(&self) -> Result<u16, &'static str> {
        if let Some(idx) = self.host.find(':') {
            self.host[idx+1..].parse::<u16>().map_err(|_| "Invalid port")
        } else {
            match self.scheme.as_str() {
                "ws" => Ok(80),
                "wss" => Ok(443),
                _ => Err("Invalid scheme"),
            }
        }
    }

    pub fn resource_name(&self) -> String {
        let mut resource_name = String::new();
        
        if let Some(path) = &self.path {
            if !path.starts_with('/') {
                resource_name.push_str("/");
            }
            resource_name.push_str(path);
        }
        
        if let Some(query) = &self.query {
            if resource_name.ends_with('/'){
                resource_name.strip_suffix('/').unwrap();
            }
            resource_name.push_str("?");
            resource_name.push_str(query);
        }
        
        resource_name
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