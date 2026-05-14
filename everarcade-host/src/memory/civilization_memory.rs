pub type Hash = [u8; 32];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CivilizationMemoryRecord {
    pub civilization_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
    pub continuity_root: Hash,
    pub epoch_index: u64,
}
