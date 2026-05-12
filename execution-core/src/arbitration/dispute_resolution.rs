use super::arbitration_case::Hash;

pub fn resolve_dispute(dispute_root: Hash, treaty_root: Hash) -> Hash {
    let mut out = dispute_root;
    for (i, b) in treaty_root.iter().enumerate() { out[i] ^= *b; }
    out
}
