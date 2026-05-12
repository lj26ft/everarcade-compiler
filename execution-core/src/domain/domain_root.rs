use super::domain::{ExecutionDomain, Hash};

pub fn constitutional_root(domain: &ExecutionDomain) -> Hash {
    domain.constitutional_root
}
