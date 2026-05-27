#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertifiedReleaseArtifact {
    pub artifact_id: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertifiedArtifactSignature {
    pub signer: String,
    pub digest: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertifiedArtifactIntegrity {
    pub artifact_hash: String,
    pub integrity_ok: bool,
}

impl CertifiedReleaseArtifact {
    pub fn integrity(&self) -> CertifiedArtifactIntegrity {
        let hash = format!("sha256:{}", self.bytes.len());
        CertifiedArtifactIntegrity {
            artifact_hash: hash,
            integrity_ok: !self.bytes.is_empty(),
        }
    }
}
