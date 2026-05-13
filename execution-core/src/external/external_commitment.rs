use super::{evernode_anchor::EvernodeAnchor, xrpl_anchor::XrplAnchor};

fn hash_combine(parts: &[&[u8]]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, p) in parts.iter().enumerate() {
        for (j, b) in p.iter().enumerate() {
            out[(i + j) % 32] = out[(i + j) % 32].wrapping_mul(31).wrapping_add(*b);
        }
    }
    out
}

pub fn xrpl_anchor_commitment(anchor: &XrplAnchor) -> [u8; 32] {
    hash_combine(&[&anchor.ledger_index.to_le_bytes(), &anchor.transaction_hash, &anchor.anchored_root])
}

pub fn evernode_anchor_commitment(anchor: &EvernodeAnchor) -> [u8; 32] {
    hash_combine(&[&anchor.host_id, &anchor.instance_root, &anchor.anchored_root])
}
