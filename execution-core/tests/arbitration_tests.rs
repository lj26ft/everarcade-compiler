use execution_core::arbitration::{arbitration_case::ArbitrationCase, dispute_resolution::resolve_dispute};
#[test]
fn arbitration_is_deterministic() {
 let case=ArbitrationCase{case_id:[1;32],participating_domains:vec![[2;32]],treaty_root:[3;32],dispute_root:[4;32],resolution_root:[0;32]};
 assert_eq!(resolve_dispute(case.dispute_root, case.treaty_root), resolve_dispute(case.dispute_root, case.treaty_root));
}
