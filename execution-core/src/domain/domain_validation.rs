use super::domain::ExecutionDomain;

pub fn validate_domain(domain: &ExecutionDomain) -> bool {
    domain.domain_id != [0; 32]
        && domain.constitutional_root != [0; 32]
        && domain.governance_root != [0; 32]
        && domain.replay_root != [0; 32]
}
