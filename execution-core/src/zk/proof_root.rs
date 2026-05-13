use sha2::{Digest, Sha256};
use super::Hash;

pub fn compute_proof_root(chunks: &[Hash]) -> Hash {
    let mut h = Sha256::new();
    for c in chunks { h.update(c); }
    h.finalize().into()
}
