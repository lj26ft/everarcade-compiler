use super::runtime_commit::CivilizationRuntimeCommit;
pub type Hash = [u8; 32];
pub fn execute_runtime_loop(governance_root: Hash, economic_root: Hash, replay_root: Hash, checkpoint_root: Hash) -> CivilizationRuntimeCommit { let civilization_root = std::array::from_fn(|i| governance_root[i] ^ economic_root[i]); CivilizationRuntimeCommit { civilization_root, replay_root, checkpoint_root, governance_root, economic_root } }
