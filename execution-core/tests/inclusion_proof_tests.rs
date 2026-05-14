use execution_core::merkle::{
    inclusion_proof::{generate_inclusion_proof, verify_inclusion_proof},
    leaf_hash::leaf_hash,
    merkle_tree::build_merkle_root,
};

#[test]
fn generated_proof_verifies_and_tamper_fails() {
    let leaves = vec![leaf_hash(b"a"), leaf_hash(b"b"), leaf_hash(b"c")];
    let root = build_merkle_root(&leaves);
    let proof = generate_inclusion_proof(&leaves, 1);
    assert!(verify_inclusion_proof(root, leaves[1], &proof));

    let mut bad = proof.clone();
    bad.siblings[0][0] ^= 0x01;
    assert!(!verify_inclusion_proof(root, leaves[1], &bad));
}
