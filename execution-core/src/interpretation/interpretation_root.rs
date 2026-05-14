use super::interpretation::{ConstitutionalInterpretation, Hash};
pub fn interpretation_root(items: &[ConstitutionalInterpretation]) -> Hash {
    let mut out = [0u8; 32];
    for item in items {
        for i in 0..32 {
            out[i] ^= item.interpretation_id[i] ^ item.lineage_root[i];
        }
    }
    out
}
