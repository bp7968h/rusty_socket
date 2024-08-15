use std::io::{Read, Write};
use std::net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};
use std::thread;
use std::sync::{Mutex, Arc};

use crate::{Connection, HandShake, SsError};
use crate::Result;

pub struct SocketServer {
    target: SocketAddrV4,
    active_connections: Arc<Mutex<Vec<TcpStream>>>,
}

impl SocketServer {
    pub fn build(address: impl ToSocketAddrs) -> Result<Self> {
        let mut addrs = address.to_socket_addrs().map_err(SsError::from)?;
        if let Some(SocketAddr::V4(target)) = addrs.next() {
            Ok( SocketServer {
                target,
                active_connections: Arc::new(Mutex::new(Vec::new())),
            })
        } else {
            Err(SsError::InvalidBindAddress)
        }
    }

    pub fn start(&self) {
        let tcp_listener = TcpListener::bind(&self.target).unwrap();

        println!("Server Listening on {}", &self.target.to_string());

        for stream in tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_connection(stream);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    panic!("Shutting Down Server Due to Error in Stream");
                }
            }
        }
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        let _ = stream.read(&mut buffer);
        let client_request = String::from_utf8_lossy(&buffer);

        let handshake = HandShake::perform(&client_request);

        match handshake.request {
            Some(_) => {
                stream
                    .write_all(handshake.response.to_string().as_bytes())
                    .expect("Failed to write success response");
                stream.flush().expect("Failed to flush success stream");

                match self.active_connections.try_lock() {
                    Ok(mut connections) => {
                        connections.push(stream.try_clone().unwrap());
                    },
                    Err(e) => {
                        panic!("Failed to lock active connections: {}", e);
                    }
                }

                let rc_active_conn = Arc::clone(&self.active_connections);
                thread::spawn(move || {
                    Connection::handle_frames(stream, rc_active_conn);
                });
            }
            None => {
                stream
                    .write_all(handshake.response.to_string().as_bytes())
                    .expect("Failed to write response");
                stream.flush().expect("Failed to flush stream");
            }
        }
    }
}
