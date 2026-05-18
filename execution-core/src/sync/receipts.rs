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
    Ok(())
}
