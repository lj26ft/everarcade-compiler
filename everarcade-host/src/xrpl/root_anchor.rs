use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XrplRootAnchorIntent {
    pub civilization_root: [u8; 32],
    pub receipt_root: [u8; 32],
    pub checkpoint_root: [u8; 32],
    pub ipfs_manifest_root: [u8; 32],
    pub proof_root: [u8; 32],
}
