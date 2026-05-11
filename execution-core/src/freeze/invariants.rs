use super::versions;

pub fn deterministic_hashing_invariant() -> &'static str { versions::HASH_VERSION }
pub fn execution_invariant() -> &'static str { versions::EXECUTION_VERSION }
pub fn replay_invariant() -> &'static str { versions::PROTOCOL_VERSION }
pub fn snapshot_invariant() -> &'static str { versions::SNAPSHOT_VERSION }
