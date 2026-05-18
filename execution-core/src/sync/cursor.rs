use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{canonical::encoding::canonical_encode, operator::continuity::Hash256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncCursor {
    pub latest_sequence: u64,
    pub latest_execution_id: Hash256,
    pub latest_checkpoint_root: Hash256,
    pub latest_manifest_hash: Hash256,
    pub latest_lineage_hash: Hash256,
}

pub fn hash_cursor(cursor: &SyncCursor) -> Hash256 {
    Sha256::digest(&canonical_encode(cursor).expect("sync cursor encode")).into()
}
