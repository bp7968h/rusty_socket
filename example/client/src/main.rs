use rusty_socket_client::SocketClient;
use std::io::Read;

fn main() {
    let client = SocketClient::build("ws://127.0.0.1:8080/chat");

    match client {
        Ok(mut data_client) => {
            let mut buffer = [0; 512];
            loop {
                // Read data from the stream
                match data_client.stream.read(&mut buffer) {
                    Ok(n) if n == 0 => {
                        println!("Connection closed by the server.");
                        break;
                    }
                    Ok(n) => {
                        let received_data = String::from_utf8_lossy(&buffer[..n]);
                        println!("Received: {}", received_data);
                    }
                    Err(e) => {
                        eprintln!("Failed to read from stream: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => println!("Failed to build SocketClient: {}", e),
    }
}
