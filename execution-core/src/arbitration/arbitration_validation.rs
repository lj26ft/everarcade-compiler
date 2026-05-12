use super::{arbitration_case::ArbitrationCase, constitutional_dispute::dispute_is_constitutional};

pub fn validate_case(case: &ArbitrationCase) -> bool {
    !case.participating_domains.is_empty() && dispute_is_constitutional(case.dispute_root)
}
