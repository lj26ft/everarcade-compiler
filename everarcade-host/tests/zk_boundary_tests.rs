use execution_core::zk::{validation::validate_artifact, ZkProofArtifact};
#[test]
fn validates_zk_boundary() {
    let a = ZkProofArtifact {
        proof_root: [1; 32],
        statement_root: [2; 32],
        verification_key_root: [3; 32],
        public_inputs_root: [4; 32],
    };
    assert!(validate_artifact(&a));
}
