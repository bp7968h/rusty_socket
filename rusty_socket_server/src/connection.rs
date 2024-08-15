use std::net::TcpStream;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use rusty_socket_core::{DataFrame, OpCode};

pub struct Connection {}

impl Connection {
    pub fn handle_frames(mut stream: TcpStream, active_conn: Arc<Mutex<Vec<TcpStream>>>) {
        let mut buffer = [0; 512];
        loop {
            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size == 0 {
                        break;
                    }

                    let received_frame: DataFrame = DataFrame::try_from(&buffer[..size]).unwrap();
                    let received_data: String = String::from_utf8(received_frame.payload).unwrap();
                    println!("Received: {}", received_data);


                    if let Some(sending_frame) =
                        DataFrame::from_data(&received_data, OpCode::Text, false)
                    {
                        let frame_bytes = Vec::from(sending_frame);

                        match active_conn.try_lock() {
                            Ok(mut connections) => {
                                for connection in connections.iter_mut() {
                                    if let Err(e) = connection.write_all(&frame_bytes) {
                                        eprintln!("Failed to write to connection: {}", e);
                                    }
                                    if let Err(e) = connection.flush() {
                                        eprintln!("Failed to flush connection: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("Failed to lock active connections: {}", e);
                                continue;
                            }
                        };

                        println!("Data broadcast to all clients");
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