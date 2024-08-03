use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::io::{Read};

use crate::Connections;

pub struct SocketServer {
    target: SocketAddrV4,
    connections: Vec<Connections>
}

impl SocketServer {
    pub fn new() -> Self {
        SocketServer{
            target: "127.0.0.1:8080".parse::<SocketAddrV4>().unwrap(),
            connections: Vec::new(),
        }
    }

    pub fn start(&self){
        let tcp_listener = TcpListener::bind(&self.target).unwrap();

        println!("Server Listeneing on {}", &self.target.to_string());

        for stream in tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    Self::handle_connection(stream);
                },
                Err(e) => {
                    println!("Error: {}", e);
                    panic!("Sutting Down Server Due to Error in Stream");
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];
        let _ = stream.read(&mut buffer);

        println!("Received: {:?}", buffer);
    }
}