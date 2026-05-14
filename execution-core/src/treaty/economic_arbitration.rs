use super::economic_treaty::Hash;
pub fn arbitrate_economic_dispute(claim_root: Hash, evidence_root: Hash) -> Hash {
    let mut o = [0; 32];
    for i in 0..32 {
        o[i] = claim_root[i] ^ evidence_root[31 - i];
    }
    o
}
