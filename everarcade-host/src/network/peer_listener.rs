use std::net::{TcpListener, TcpStream};

use crate::network::{message_framing::*, network_error::NetworkError};

pub fn serve_once(
    listener: &TcpListener,
    response_payload: &[u8],
) -> Result<Vec<u8>, NetworkError> {
    let (mut stream, _) = listener.accept()?;
    let request = read_frame(&mut stream)?;
    write_frame(&mut stream, response_payload)?;
    Ok(request)
}

pub fn read_payload(stream: &mut TcpStream) -> Result<Vec<u8>, NetworkError> {
    read_frame(stream)
}
