use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayVerificationResult {
    pub valid: bool,
    pub continuity_digest: String,
    pub archive_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayVerificationFailure {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct ReplayProofVerificationRuntime;

impl ReplayProofVerificationRuntime {
    pub fn verify(
        ancestry_chain: &[String],
        expected_continuity_digest: &str,
        archive_payload: &[u8],
        expected_archive_digest: &str,
    ) -> Result<ReplayVerificationResult, ReplayVerificationFailure> {
        if ancestry_chain.is_empty() {
            return Err(ReplayVerificationFailure {
                code: "empty_ancestry".into(),
                message: "replay ancestry cannot be empty".into(),
            });
        }

        let continuity_digest = Self::digest_chain(ancestry_chain);
        let archive_digest = Self::digest_bytes(archive_payload);

        if continuity_digest != expected_continuity_digest {
            return Err(ReplayVerificationFailure {
                code: "continuity_tampering".into(),
                message: "continuity lineage digest mismatch".into(),
            });
        }
        if archive_digest != expected_archive_digest {
            return Err(ReplayVerificationFailure {
                code: "archive_ancestry_mismatch".into(),
                message: "archive proof digest mismatch".into(),
            });
        }

        Ok(ReplayVerificationResult {
            valid: true,
            continuity_digest,
            archive_digest,
        })
    }

    pub fn digest_chain(ancestry_chain: &[String]) -> String {
        let mut hasher = Sha256::new();
        for node in ancestry_chain {
            hasher.update(node.as_bytes());
            hasher.update([0xff]);
        }
        format!("{:x}", hasher.finalize())
    }

    pub fn digest_bytes(payload: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload);
        format!("{:x}", hasher.finalize())
    }
}
