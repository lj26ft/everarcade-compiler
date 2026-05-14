pub type Hash = [u8; 32];

use sha2::{Digest, Sha256};

pub fn extend_identity_lineage(previous_lineage_root: Hash, identity_root: Hash) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(previous_lineage_root);
    hasher.update(identity_root);
    hasher.finalize().into()
}
