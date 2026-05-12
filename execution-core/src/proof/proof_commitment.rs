use sha2::{Digest, Sha256};

use super::aggregated_proof::Hash;

pub fn proof_commitment(mut roots: Vec<Hash>) -> Hash {
    roots.sort();
    let mut hasher = Sha256::new();
    for root in roots { hasher.update(root); }
    hasher.finalize().into()
}
