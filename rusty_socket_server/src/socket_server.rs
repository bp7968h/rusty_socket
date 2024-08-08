use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::io::{Read, Write};

use rusty_socket_core::{HandShake, DataFrame, OpCode};
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
                    self.handle_connection(stream);
                },
                Err(e) => {
                    println!("Error: {}", e);
                    panic!("Sutting Down Server Due to Error in Stream");
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        let _ = stream.read(&mut buffer);
        let client_request = String::from_utf8_lossy(&buffer);

        let handshake = HandShake::build(&client_request);


        match handshake.request {
            Some(_request) => {
                stream.write_all(handshake.response.to_string().as_bytes()).expect("Failed to write success response");
                stream.flush().expect("Failed to flush success stream");

                self.handle_frames(stream);
            },
            None => {
                stream.write_all(handshake.response.to_string().as_bytes()).expect("Failed to write response");
                stream.flush().expect("Failed to flush stream");
            }
        }
    }

    fn handle_frames(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        loop {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        break;
                    }
                    println!("WebSocket frame received: {:?}", &buffer[..size]);
                    let received_frame: DataFrame = DataFrame::try_from(&buffer[..size]).unwrap();
                    let received_data : String =  String::from_utf8(received_frame.payload).unwrap();
                    println!("Received: {}", received_data);

                    if let Some(sending_frame) = DataFrame::from_data(&received_data, OpCode::Text, false) {
                        stream.write_all(&Vec::from(sending_frame)).unwrap();
                        stream.flush().unwrap();
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read WebSocket frame: {}", e);
                    break;
                }
            }
        }
    }
}