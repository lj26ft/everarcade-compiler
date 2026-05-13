use super::{arbitration_case::ArbitrationCase, constitutional_review::constitutional_review};

pub fn court_reduce(case: &ArbitrationCase) -> [u8; 32] {
    constitutional_review(case)
}
