use super::domain::ExecutionDomain;

pub fn is_same_domain(left: &ExecutionDomain, right: &ExecutionDomain) -> bool {
    left.domain_id == right.domain_id
}
