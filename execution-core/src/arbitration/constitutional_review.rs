use super::arbitration_case::ArbitrationCase;

pub fn constitutional_review(case: &ArbitrationCase) -> [u8; 32] {
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = case.case_id[i] ^ case.resolution_root[i] ^ case.dispute_root[i];
    }
    out
}
