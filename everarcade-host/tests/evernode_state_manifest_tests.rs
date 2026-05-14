use everarcade_host::evernode::state_manifest::EvernodeStateManifest;
#[test]
fn builds_evernode_manifest() {
    let m = EvernodeStateManifest {
        package_root: [1; 32],
        vm_receipt_root: [2; 32],
        checkpoint_root: [3; 32],
        ipfs_manifest_root: [4; 32],
        xrpl_anchor_root: [5; 32],
        evernode_instance_root: [6; 32],
    };
    assert_eq!(m.package_root, [1; 32]);
}
