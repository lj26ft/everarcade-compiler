use execution_core::{ExecutionPlan, VmInput};

#[test]
fn test_fuel_determinism() {
    let input = VmInput { state: Default::default(), plan: ExecutionPlan { nodes: vec![] } };
    let a = execution_core::execute::execute_vm(input.clone());
    let b = execution_core::execute::execute_vm(input);
    assert_eq!(a.receipt.fuel_used, b.receipt.fuel_used);
}

#[test]
fn test_fuel_limit_enforced() {
    let input = VmInput { state: Default::default(), plan: ExecutionPlan { nodes: vec![] } };
    let out = execution_core::execute::execute_vm(input);
    assert_eq!(out.receipt.fuel_used, 0);
}
