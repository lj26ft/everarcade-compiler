use execution_core::tenancy::{tenant::ExecutionTenant, tenant_governance::shares_domain};

#[test]
fn tenant_isolation_requires_same_domain_for_shared_governance() {
    let a = ExecutionTenant {
        tenant_id: [1; 32],
        domain_id: [2; 32],
        budget_root: [3; 32],
        replay_root: [4; 32],
    };
    let b = ExecutionTenant {
        tenant_id: [5; 32],
        domain_id: [6; 32],
        budget_root: [7; 32],
        replay_root: [8; 32],
    };
    assert!(!shares_domain(&a, &b));
}
