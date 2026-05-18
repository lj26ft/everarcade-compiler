use serde::{Deserialize, Serialize};

use crate::{federation::node::FederationNodeId, operator::continuity::Hash256};

use super::{
    cursor::SyncCursor,
    errors::{Result, SyncError},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityAdvertisement {
    pub world_id: String,
    pub operator: FederationNodeId,
    pub cursor: SyncCursor,
    pub package_root: Hash256,
    pub checkpoint_root: Hash256,
}

pub fn verify_advertisement(a: &ContinuityAdvertisement) -> Result<()> {
    if a.world_id.trim().is_empty() {
        return Err(SyncError::mismatch("world_id", "non-empty", "empty"));
    }
    if a.package_root == [0u8; 32] {
        return Err(SyncError::mismatch("package_root", "non-zero", "zero"));
    }
    if a.checkpoint_root == [0u8; 32] {
        return Err(SyncError::mismatch("checkpoint_root", "non-zero", "zero"));
    }
    if a.cursor.latest_checkpoint_root != a.checkpoint_root {
        return Err(SyncError::mismatch(
            "checkpoint_root",
            hex::encode(a.cursor.latest_checkpoint_root),
            hex::encode(a.checkpoint_root),
        ));
    }
    Ok(())
}
