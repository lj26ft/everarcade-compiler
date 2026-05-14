pub type Hash = [u8; 32];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CivilizationArchivePackage {
    pub archive_root: Hash,
    pub civilization_root: Hash,
    pub replay_summary_root: Hash,
    pub checkpoint_root: Hash,
    pub continuity_root: Hash,
}
