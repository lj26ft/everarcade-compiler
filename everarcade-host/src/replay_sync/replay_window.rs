pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayWindow {
    pub start_receipt_root: Hash,
    pub end_receipt_root: Hash,
    pub receipt_count: u64,
    pub replay_root: Hash,
}
