pub type Hash = [u8; 32];
pub fn divergence_detected(local_root: Hash, remote_root: Hash) -> bool { local_root != remote_root }
