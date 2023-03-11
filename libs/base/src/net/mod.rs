
mod peer;
pub use peer::*;

//mod socket_listener;
//pub use socket_listener::*;

mod websocket_listener;
pub use websocket_listener::*;

pub enum Data {
    Binary(bytes::Bytes),
    Text(String),
    Close,
}