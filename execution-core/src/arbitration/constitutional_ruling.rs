use super::arbitration_case::ArbitrationCase;

pub fn constitutional_ruling(case: &ArbitrationCase, review_root: [u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for i in 0..32 { out[i] = case.treaty_root[i] ^ review_root[i]; }
    out
}
