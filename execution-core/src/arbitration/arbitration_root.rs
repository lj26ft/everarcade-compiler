use super::arbitration_case::{ArbitrationCase, Hash};

pub fn arbitration_root(case: &ArbitrationCase) -> Hash {
    let mut out = case.treaty_root;
    for (i, b) in case.dispute_root.iter().enumerate() {
        out[i] ^= *b;
    }
    for (i, b) in case.resolution_root.iter().enumerate() {
        out[i] ^= *b;
    }
    out
}
