use std::net::TcpStream;

use super::network_error::NetworkError;

pub fn connect(addr: &str) -> Result<TcpStream, NetworkError> {
    Ok(TcpStream::connect(addr)?)
}
