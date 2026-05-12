use super::tenant::ExecutionTenant;

pub fn shares_domain(left: &ExecutionTenant, right: &ExecutionTenant) -> bool {
    left.domain_id == right.domain_id
}
