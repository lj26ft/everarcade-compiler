use everarcade_host::xrpl::{memo_payload::memo_payload_hash, root_anchor::XrplRootAnchorIntent};
#[test]
fn builds_xrpl_root_anchor() {
    let i = XrplRootAnchorIntent {
        civilization_root: [1; 32],
        receipt_root: [2; 32],
        checkpoint_root: [3; 32],
        ipfs_manifest_root: [4; 32],
        proof_root: [5; 32],
    };
    let h = memo_payload_hash(&i);
    assert_ne!(h, [0; 32]);
}
