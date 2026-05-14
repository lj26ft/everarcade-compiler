pub type Hash = [u8; 32];

use sha2::{Digest, Sha256};

pub fn derive_peer_fingerprint(node_id: Hash, public_key_root: Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(node_id);
    hasher.update(public_key_root);
    hasher.finalize().into()
}
