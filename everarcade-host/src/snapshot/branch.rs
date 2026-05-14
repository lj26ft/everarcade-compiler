pub type Hash = [u8; 32];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CivilizationBranch {
    pub parent_replay_root: Hash,
    pub branch_replay_root: Hash,
    pub continuity_root: Hash,
}
