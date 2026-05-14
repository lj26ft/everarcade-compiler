pub type Hash = [u8; 32];
pub fn scope_compatible(local: Option<Hash>, remote: Option<Hash>) -> bool {
    local == remote
}
