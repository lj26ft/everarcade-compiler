use super::{evernode_anchor::EvernodeAnchor, xrpl_anchor::XrplAnchor};

pub fn validate_xrpl_anchor(anchor: &XrplAnchor) -> bool {
    anchor.ledger_index > 0 && anchor.anchored_root != [0u8; 32]
}

pub fn validate_evernode_anchor(anchor: &EvernodeAnchor) -> bool {
    anchor.host_id != [0u8; 32] && anchor.anchored_root != [0u8; 32]
}
