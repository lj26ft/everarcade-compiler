use super::domain::{ExecutionDomain, Hash};

pub fn domain_identity(domain: &ExecutionDomain) -> Hash {
    domain.domain_id
}
