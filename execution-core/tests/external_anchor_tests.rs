use execution_core::external::{external_commitment::{evernode_anchor_commitment, xrpl_anchor_commitment}, EvernodeAnchor, XrplAnchor};
#[test]
fn anchor_commitments_deterministic() {
 let x = XrplAnchor{ledger_index:1, transaction_hash:[2;32], anchored_root:[3;32]};
 assert_eq!(xrpl_anchor_commitment(&x), xrpl_anchor_commitment(&x));
 let e = EvernodeAnchor{host_id:[4;32], instance_root:[5;32], anchored_root:[6;32]};
 assert_eq!(evernode_anchor_commitment(&e), evernode_anchor_commitment(&e));
}
