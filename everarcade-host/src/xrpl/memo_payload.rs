use sha2::{Digest,Sha256}; use super::root_anchor::XrplRootAnchorIntent;
pub fn memo_payload_hash(i:&XrplRootAnchorIntent)->[u8;32]{Sha256::digest(serde_json::to_vec(i).unwrap()).into()}
