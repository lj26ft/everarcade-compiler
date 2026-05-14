use super::precedent::{Hash, LegalPrecedent};
pub fn jurisprudence_root(precedents: &[LegalPrecedent]) -> Hash {
    let mut out = [0u8; 32];
    for p in precedents {
        for i in 0..32 {
            out[i] ^= p.precedent_id[i] ^ p.interpretation_root[i] ^ p.lineage_root[i];
        }
    }
    out
}
