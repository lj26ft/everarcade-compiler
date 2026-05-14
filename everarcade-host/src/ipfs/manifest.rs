use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpfsManifestRecord {
    pub manifest_root: [u8; 32],
    pub artifact_count: u32,
}
