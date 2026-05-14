pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreatyExecutionCommit {
    pub treaty_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
}
