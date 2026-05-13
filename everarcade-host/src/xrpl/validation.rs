use super::anchor_intent::XrplAnchorIntent;

pub fn validate_intent(intent: &XrplAnchorIntent) -> bool { !intent.receipt_id_hex.is_empty() && !intent.payload_hex.is_empty() }
