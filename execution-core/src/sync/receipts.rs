use crate::vm::VmExecutionReceipt;

use super::{
    errors::{Result, SyncError},
    window::SyncWindow,
};

pub fn verify_receipt_window(receipts: &[VmExecutionReceipt], window: &SyncWindow) -> Result<()> {
    let expected = (window.end_sequence - window.start_sequence + 1) as usize;
    if receipts.len() != expected {
        return Err(SyncError::mismatch(
            "receipts",
            expected.to_string(),
            receipts.len().to_string(),
        ));
    }
    for i in 1..receipts.len() {
        if receipts[i].prior_replay_root != receipts[i - 1].post_state_root {
            return Err(SyncError::mismatch(
                "replay_root",
                hex::encode(receipts[i - 1].post_state_root),
                hex::encode(receipts[i].prior_replay_root),
            ));
        }
        if receipts[i].pre_state_root != receipts[i - 1].post_state_root {
            return Err(SyncError::mismatch(
                "state_root",
                hex::encode(receipts[i - 1].post_state_root),
                hex::encode(receipts[i].pre_state_root),
            ));
        }
    }
    Ok(())
}
