use sha2::{Digest, Sha256};

use super::anchor_receipt::{ExternalAnchorReceipt, Hash};

pub fn emit_external_anchor_receipt(anchor_root: Hash) -> ExternalAnchorReceipt {
    let xrpl_anchor_root = Some(Sha256::digest([anchor_root.as_slice(), b"xrpl"].concat()).into());
    let evernode_anchor_root = Some(Sha256::digest([anchor_root.as_slice(), b"evernode"].concat()).into());
    ExternalAnchorReceipt { anchor_root, xrpl_anchor_root, evernode_anchor_root }
}
