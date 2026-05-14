use execution_core::merkle::{
    inclusion_proof::generate_inclusion_proof, leaf_hash::leaf_hash, merkle_tree::build_merkle_root,
};
use execution_core::replay::{proof_validator::validate_replay_proof, replay_proof::ReplayProof};

#[test]
fn replay_proof_validation_works() {
    let leaves = vec![leaf_hash(b"r0"), leaf_hash(b"r1")];
    let root = build_merkle_root(&leaves);
    let proof = ReplayProof {
        receipt_root: root,
        leaf_hash: leaves[1],
        inclusion_proof: generate_inclusion_proof(&leaves, 1),
    };
    assert!(validate_replay_proof(&proof));
}
