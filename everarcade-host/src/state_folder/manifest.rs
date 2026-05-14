use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateFolderManifest {
    pub receipt_root: [u8; 32],
    pub checkpoint_root: [u8; 32],
}
