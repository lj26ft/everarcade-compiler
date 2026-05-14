use std::net::TcpStream;

use crate::network::{message_framing::*, network_error::NetworkError};

pub fn request_sync(mut stream: TcpStream, payload: &[u8]) -> Result<Vec<u8>, NetworkError> {
    write_frame(&mut stream, payload)?;
    read_frame(&mut stream)
}
