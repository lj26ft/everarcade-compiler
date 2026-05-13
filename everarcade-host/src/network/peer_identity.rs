use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];

pub fn derive_peer_id(node_root: Hash, checkpoint_root: Hash) -> Hash {
    let mut h = Sha256::new();
    h.update(node_root);
    h.update(checkpoint_root);
    h.finalize().into()
}
