#[cfg(feature = "ipfs-live")]
pub fn publish_bytes(bytes: &[u8]) -> Option<String> {
    Some(format!("cid:{}", bytes.len()))
}

#[cfg(not(feature = "ipfs-live"))]
pub fn publish_bytes(_bytes: &[u8]) -> Option<String> {
    None
}
