use execution_core::{ExecutionPlan, VmInput};
use std::collections::BTreeMap;

#[test]
fn deterministic_bincode_roundtrip() {
    let input = VmInput { state: BTreeMap::new(), plan: ExecutionPlan { nodes: vec![] } };
    let bytes = everarcade_abi::serialize(&input).unwrap();
    let decoded: VmInput = everarcade_abi::deserialize(&bytes).unwrap();
    let bytes2 = everarcade_abi::serialize(&decoded).unwrap();
    assert_eq!(bytes, bytes2);
}
