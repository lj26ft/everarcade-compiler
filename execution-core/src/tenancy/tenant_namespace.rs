use super::tenant::{ExecutionTenant, Hash};

pub fn tenant_namespace_root(tenant: &ExecutionTenant) -> Hash {
    tenant.tenant_id
}
