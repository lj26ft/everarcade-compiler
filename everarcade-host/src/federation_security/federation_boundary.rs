pub type Hash = [u8; 32];
pub fn federation_boundary_preserved(local_scope: Hash, remote_scope: Hash) -> bool {
    local_scope == remote_scope
}
