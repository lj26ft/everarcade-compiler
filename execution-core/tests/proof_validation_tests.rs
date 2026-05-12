use execution_core::merkle::{inclusion_proof::generate_inclusion_proof, leaf_hash::leaf_hash, merkle_tree::build_merkle_root, proof_validation::validate_proof};

#[test]
fn validate_proof_path() {
    let leaves = vec![leaf_hash(b"a"), leaf_hash(b"b")];
    let root = build_merkle_root(&leaves);
    let proof = generate_inclusion_proof(&leaves, 0);
    assert!(validate_proof(root, leaves[0], &proof));
}
