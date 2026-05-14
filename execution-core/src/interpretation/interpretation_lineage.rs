use super::interpretation::Hash;
pub fn interpretation_lineage(prior: Hash, next: Hash) -> Hash {
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = prior[i] ^ next[i] ^ 0x55;
    }
    out
}
