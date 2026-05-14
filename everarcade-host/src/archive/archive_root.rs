pub type Hash = [u8; 32];
use sha2::{Digest, Sha256};
pub fn hash_roots(roots: &[Hash]) -> Hash {
    let mut h = Sha256::new();
    for root in roots {
        h.update(root);
    }
    h.finalize().into()
}
pub fn archive_root(
    civilization_root: Hash,
    replay_summary_root: Hash,
    checkpoint_root: Hash,
    continuity_root: Hash,
) -> Hash {
    hash_roots(&[
        civilization_root,
        replay_summary_root,
        checkpoint_root,
        continuity_root,
    ])
}
