use super::tenant::{ExecutionTenant, Hash};

pub fn tenant_identity(tenant: &ExecutionTenant) -> Hash {
    tenant.tenant_id
}
