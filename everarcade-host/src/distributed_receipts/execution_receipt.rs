pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DistributedExecutionReceipt {
    pub receipt_root: Hash,
    pub task_root: Hash,
    pub package_root: Hash,
    pub operator_id: Hash,
}
