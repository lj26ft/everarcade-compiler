use serde::{Deserialize, Serialize};
pub type Hash = [u8; 32];
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IpfsPublicationIntent {
    pub artifact_root: Hash,
    pub artifact_path: String,
    pub cid: Option<String>,
}
