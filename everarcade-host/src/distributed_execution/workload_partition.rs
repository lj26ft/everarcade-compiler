pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkloadPartition {
    pub partition_id: Hash,
    pub package_root: Hash,
    pub partition_root: Hash,
    pub assigned_operator: Hash,
    pub execution_window: Hash,
}
