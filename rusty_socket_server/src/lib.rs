pub mod connection;
pub mod socket_server;
pub mod errors;
pub mod handshake;


pub use socket_server::SocketServer;
pub use errors::SsError;
pub use handshake::{HandShake, RequestLine, ResponseLine};
pub use connection::Connection;

pub type Result<T> = std::result::Result<T, SsError>;
