pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DistributedExecutionWindow {
    pub window_id: Hash,
    pub package_root: Hash,
    pub assigned_operator: Hash,
    pub execution_root: Hash,
    pub replay_root: Hash,
}
