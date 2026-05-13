pub mod proof_artifact;
pub mod proof_intent;
pub mod proof_root;
pub mod validation;
pub mod verification_key;

pub type Hash = [u8; 32];

pub use proof_artifact::ZkProofArtifact;
