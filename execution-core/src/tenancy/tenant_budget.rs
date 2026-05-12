use super::tenant::ExecutionTenant;

pub fn tenant_budget_root(tenant: &ExecutionTenant) -> [u8; 32] {
    tenant.budget_root
}
