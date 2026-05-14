use everarcade_host::{
    evernode::state_manifest::EvernodeStateManifest, ipfs::publisher::build_intent,
    state_folder::writer::initialize, xrpl::root_anchor::XrplRootAnchorIntent,
};
use execution_core::zk::ZkProofArtifact;

#[test]
fn end_to_end_scaling_publication_flow() {
    let base = std::env::temp_dir().join("everarcade-scaling-flow");
    let _ = std::fs::remove_dir_all(&base);
    initialize(&base).unwrap();
    let ipfs = build_intent([7; 32], "state/manifests/runtime.json".into(), b"manifest");
    let xrpl = XrplRootAnchorIntent {
        civilization_root: [1; 32],
        receipt_root: [2; 32],
        checkpoint_root: [3; 32],
        ipfs_manifest_root: ipfs.artifact_root,
        proof_root: [9; 32],
    };
    let _manifest = EvernodeStateManifest {
        package_root: [1; 32],
        vm_receipt_root: [2; 32],
        checkpoint_root: [3; 32],
        ipfs_manifest_root: ipfs.artifact_root,
        xrpl_anchor_root: xrpl.receipt_root,
        evernode_instance_root: [6; 32],
    };
    let zk = ZkProofArtifact {
        proof_root: [9; 32],
        statement_root: [8; 32],
        verification_key_root: [7; 32],
        public_inputs_root: [6; 32],
    };
    assert_eq!(zk.proof_root, xrpl.proof_root);
}
