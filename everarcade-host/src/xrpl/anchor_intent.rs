use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct XrplAnchorIntent {
    pub receipt_id_hex: String,
    pub anchor_root_hex: String,
    pub payload_hex: String,
}
