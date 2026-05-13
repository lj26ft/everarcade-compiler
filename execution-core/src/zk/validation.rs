use super::proof_artifact::ZkProofArtifact;

pub fn validate_artifact(a: &ZkProofArtifact) -> bool {
    a.proof_root != [0; 32] && a.statement_root != [0; 32]
}
