use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XrplObjectManifest {
    pub manifest_id: String,
    pub object_type: String,
    pub object_hash: String,
}
