use super::amendment::{ConstitutionalAmendment, Hash};
pub fn amendment_root(amendments: &[ConstitutionalAmendment]) -> Hash {
    let mut out = [0u8; 32];
    for amendment in amendments { for i in 0..32 { out[i] ^= amendment.amendment_id[i] ^ amendment.amendment_lineage_root[i]; } }
    out
}
