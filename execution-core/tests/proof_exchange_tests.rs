use execution_core::{
    merkle::{
        inclusion_proof::generate_inclusion_proof, leaf_hash::leaf_hash,
        merkle_tree::build_merkle_root,
    },
    sync::{validate_proof_exchange, ProofExchange, StateProof},
};

#[test]
fn proof_exchange_rejects_tampered_proofs() {
    let leaves = vec![leaf_hash(b"a"), leaf_hash(b"b")];
    let root = build_merkle_root(&leaves);
    let mut proof = generate_inclusion_proof(&leaves, 0);
    proof.siblings[0] = [9; 32];
    let ex = ProofExchange {
        state_proofs: vec![StateProof {
            root,
            leaf: leaves[0],
            proof,
        }],
        receipt_proofs: vec![],
        replay_proof: None,
        checkpoint: None,
    };
    assert!(!validate_proof_exchange(&ex));
}
