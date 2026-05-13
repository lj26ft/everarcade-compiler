pub fn deterministic_resolution(local: [u8; 32], remote: [u8; 32]) -> [u8; 32] {
    if local <= remote { local } else { remote }
}
