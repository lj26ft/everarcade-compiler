use super::clause::Hash;
pub fn derive_lineage_root(prior_lineage_root: Hash, transition_root: Hash) -> Hash {
    let mut out = [0u8; 32];
    for i in 0..32 { out[i] = prior_lineage_root[i].wrapping_add(transition_root[i]).wrapping_add((31 - i) as u8); }
    out
}
