use rusty_socket_client::SocketClient;
use std::io::Read;

fn main() {
    let client = SocketClient::build("ws://127.0.0.1:8080/chat");

    match client {
        Ok(_) => {
            println!("Handshake Success");
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
