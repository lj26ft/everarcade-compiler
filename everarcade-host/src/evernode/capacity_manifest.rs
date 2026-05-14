pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionCapacityManifest {
    pub operator_id: Hash,
    pub supported_package_root: Hash,
    pub max_execution_windows: u64,
    pub latest_checkpoint_root: Hash,
}
