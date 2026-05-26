#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseArtifactSignature {
    pub artifact_hash: String,
    pub signature: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseArtifactVerification {
    pub valid: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseSigningManifest {
    pub signer: String,
    pub signatures: Vec<ReleaseArtifactSignature>,
}
impl ReleaseArtifactSignature {
    pub fn verify(&self) -> ReleaseArtifactVerification {
        ReleaseArtifactVerification {
            valid: self.signature == format!("sig:{}", self.artifact_hash),
        }
    }
}
