use super::tenant::ExecutionTenant;

pub fn validate_tenant(tenant: &ExecutionTenant) -> bool {
    tenant.tenant_id != [0; 32]
        && tenant.domain_id != [0; 32]
        && tenant.budget_root != [0; 32]
        && tenant.replay_root != [0; 32]
}
