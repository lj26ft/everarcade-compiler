use super::precedent::Hash;
pub fn legal_lineage_root(prior: Hash, precedent: Hash) -> Hash {
    let mut out = [0u8; 32];
    for i in 0..32 { out[i] = prior[i].wrapping_mul(3).wrapping_add(precedent[i]); }
    out
}
