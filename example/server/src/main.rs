use rusty_socket_server::SocketServer;

fn main() {
    match SocketServer::build("127.0.0.1:8080") {
       Ok(server) => {
           server.start();
       },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
