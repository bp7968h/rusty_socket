use rusty_socket_core::HandShake;

use std::net::TcpStream;

pub struct Connections {
    handshake: HandShake,
    streams: TcpStream,
}

impl Connections {
    // pub fn build(client_stream : TcpStream) -> Self {
    //     Connections {
    //         handshake:
    //     }
    // }
}
