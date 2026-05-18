use crate::lineage::{validate_lineage_chain, ExecutionLineageChain};

use super::{
    errors::{Result, SyncError},
    window::SyncWindow,
};

pub fn verify_lineage_window(chain: &ExecutionLineageChain, window: &SyncWindow) -> Result<()> {
    validate_lineage_chain(chain)
        .map_err(|e| SyncError::mismatch("lineage", "valid", e.to_string()))?;
    let first = chain
        .records
        .first()
        .ok_or_else(|| SyncError::mismatch("lineage", "non-empty", "empty"))?;
    let last = chain.records.last().unwrap();
    if first.sequence != window.start_sequence || last.sequence != window.end_sequence {
        return Err(SyncError::mismatch(
            "sequence_window",
            format!("{}..{}", window.start_sequence, window.end_sequence),
            format!("{}..{}", first.sequence, last.sequence),
        ));
    }
    Ok(())
}
