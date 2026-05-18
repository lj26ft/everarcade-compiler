use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    federation::bundle::verify_continuity_bundle,
    operator::{recover_world, OperatorRecoveryInput},
};

use super::errors::{Result, SyncError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncVerificationReport {
    pub receipts_ok: bool,
    pub checkpoint_ok: bool,
    pub lineage_ok: bool,
    pub replay_ok: bool,
    pub continuity_ok: bool,
}

pub fn verify_sync_artifacts(bundle_root: &Path) -> Result<SyncVerificationReport> {
    let v = verify_continuity_bundle(bundle_root)
        .map_err(|e| SyncError::mismatch("bundle", "valid", e.to_string()))?;
    Ok(SyncVerificationReport {
        receipts_ok: v.receipts_ok,
        checkpoint_ok: v.checkpoint_ok,
        lineage_ok: v.lineage_ok,
        replay_ok: v.recovery_ok,
        continuity_ok: v.bundle_ok,
    })
}

pub fn verify_sync_recovery(input: OperatorRecoveryInput) -> Result<()> {
    recover_world(input).map_err(|e| SyncError::mismatch("recover_world", "ok", e.to_string()))?;
    Ok(())
}
