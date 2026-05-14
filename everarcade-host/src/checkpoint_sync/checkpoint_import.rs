use super::checkpoint_export::TransferCheckpoint;

pub fn import_checkpoint_bytes(bytes: &[u8]) -> Option<TransferCheckpoint> {
    serde_json::from_slice(bytes).ok()
}
