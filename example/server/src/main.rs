use rusty_socket_server::SocketServer;

fn main() {
    let server = SocketServer::new();

    server.start();
}
