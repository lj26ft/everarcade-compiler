use super::tenant::ExecutionTenant;

pub fn tenant_replay_root(tenant: &ExecutionTenant) -> [u8; 32] {
    tenant.replay_root
}
