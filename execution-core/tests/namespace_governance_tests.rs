use execution_core::namespace::{namespace::ExecutionNamespace, namespace_transition::transfer_namespace};

#[test]
fn namespace_transfer_updates_owner() {
    let namespace = ExecutionNamespace { namespace_id: [1; 32], owning_domain: [2; 32], delegation_root: [3; 32] };
    let moved = transfer_namespace(&namespace, [4; 32]);
    assert_eq!(moved.owning_domain, [4; 32]);
}
