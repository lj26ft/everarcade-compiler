use super::anchor_receipt::ExternalAnchorReceipt;

pub fn validate_external_anchor_receipt(receipt: &ExternalAnchorReceipt) -> bool {
    receipt.anchor_root != [0; 32]
        && (receipt.xrpl_anchor_root.is_some() || receipt.evernode_anchor_root.is_some())
}
