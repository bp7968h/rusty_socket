use rusty_socket_core::HandShake;

use std::net::{TcpStream};

pub struct Connections{
    handshake: HandShake,
    streams: TcpStream,
}