pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CivilizationRuntimeCommit {
    pub civilization_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
    pub governance_root: Hash,
    pub economic_root: Hash,
}
