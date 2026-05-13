use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvernodeAnchorIntent { pub receipt_id_hex: String, pub manifest_hash_hex: String }
