use crate::hashing::hash_bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceAction { pub actor: String, pub action: String }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GovernanceState { pub history_hash: String, pub action_count: u64 }

impl GovernanceState {
    pub fn apply(actions: &[GovernanceAction]) -> Self {
        let mut seed = String::new();
        for action in actions {
            seed.push_str(&action.actor);
            seed.push(':');
            seed.push_str(&action.action);
            seed.push(';');
        }
        Self { history_hash: hash_bytes(seed.as_bytes()), action_count: actions.len() as u64 }
    }
}
