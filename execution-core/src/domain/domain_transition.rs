use super::domain::ExecutionDomain;

pub fn transition_replay_root(domain: &ExecutionDomain, replay_root: [u8; 32]) -> ExecutionDomain {
    let mut next = domain.clone();
    next.replay_root = replay_root;
    next
}
