pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DistributedReceiptMessage {
    pub receipt_root: Hash,
    pub partition_root: Hash,
}
