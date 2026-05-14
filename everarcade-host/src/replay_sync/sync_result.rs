#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyncResult {
    pub imported_receipts: usize,
    pub replay_verified: bool,
}
