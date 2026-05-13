use super::treasury::Hash;

pub fn derive_treasury_lineage_root(previous_lineage: Hash, transition_root: Hash) -> Hash {
    let mut out = [0u8; 32];
    for i in 0..32 { out[i] = previous_lineage[i].wrapping_add(transition_root[i]); }
    out
}
