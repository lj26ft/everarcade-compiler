pub fn checkpoint_converged(local: [u8; 32], remote: [u8; 32]) -> bool {
    local == remote
}
