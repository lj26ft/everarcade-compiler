pub type Hash = [u8; 32];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconomicExecutionCommit {
    pub economic_root: Hash,
    pub treasury_root: Hash,
    pub replay_root: Hash,
    pub checkpoint_root: Hash,
}
