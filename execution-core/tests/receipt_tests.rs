use execution_core::{hashing, ExecutionPlan, VmInput};
use std::collections::BTreeMap;

#[test]
fn receipt_hash_is_stable() {
    let input = VmInput { state: BTreeMap::new(), plan: ExecutionPlan { nodes: vec![] } };
    let out = execution_core::execute::execute_vm(input);
    let h1 = hashing::compute_receipt_hash(&out.receipt);
    let h2 = hashing::compute_receipt_hash(&out.receipt);
    assert_eq!(h1, h2);
}
