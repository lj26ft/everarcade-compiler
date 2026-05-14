use std::net::TcpListener;

use super::network_error::NetworkError;

pub fn bind(addr: &str) -> Result<TcpListener, NetworkError> {
    Ok(TcpListener::bind(addr)?)
}
