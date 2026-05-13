use execution_core::arbitration::{arbitration_case::ArbitrationCase, constitutional_review::constitutional_review};

#[test]
fn judicial_determinism() {
    let case = ArbitrationCase { case_id:[1;32], participating_domains: vec![[2;32]], treaty_root:[3;32], dispute_root:[4;32], resolution_root:[5;32] };
    assert_eq!(constitutional_review(&case), constitutional_review(&case));
}
