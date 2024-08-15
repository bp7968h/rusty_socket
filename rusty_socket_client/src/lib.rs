pub mod socket_client;
pub mod errors;
pub mod url;
pub mod utils;

pub use socket_client::SocketClient;
pub use errors::ScError;
pub use url::WebSocketUrl;

pub type Result<T> = std::result::Result<T, ScError>;