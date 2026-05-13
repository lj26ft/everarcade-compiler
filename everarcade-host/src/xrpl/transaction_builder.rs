use sha2::{Digest, Sha256};

use super::anchor_intent::XrplAnchorIntent;

pub fn build_anchor_intent(receipt_id: [u8;32], anchor_root: [u8;32]) -> XrplAnchorIntent {
    let payload: [u8;32] = Sha256::digest([receipt_id.as_slice(), anchor_root.as_slice()].concat()).into();
    XrplAnchorIntent { receipt_id_hex: hex::encode(receipt_id), anchor_root_hex: hex::encode(anchor_root), payload_hex: hex::encode(payload) }
}
