use super::{bundle_builder::DeploymentBundleBuilder, hash_hex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseContinuityManifest {
    pub release_hash: String,
    pub compatibility_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseGenerationReceipt {
    pub release_hash: String,
    pub bundle_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseVerificationReceipt {
    pub verified: bool,
    pub verification_hash: String,
}

pub struct ReleasePipeline;
impl ReleasePipeline {
    pub fn generate(
        version: &str,
        archive_path: &str,
    ) -> (ReleaseGenerationReceipt, ReleaseContinuityManifest) {
        let (_, bundle) = DeploymentBundleBuilder::generate(
            vec![("version".to_string(), version.as_bytes().to_vec())],
            archive_path,
        );
        let release_hash = hash_hex(format!("{}:{}", version, bundle.bundle_hash));
        (
            ReleaseGenerationReceipt {
                release_hash: release_hash.clone(),
                bundle_hash: bundle.bundle_hash,
            },
            ReleaseContinuityManifest {
                release_hash: release_hash.clone(),
                compatibility_hash: hash_hex(version),
            },
        )
    }
    pub fn verify(manifest: &ReleaseContinuityManifest) -> ReleaseVerificationReceipt {
        ReleaseVerificationReceipt {
            verified: !manifest.release_hash.is_empty(),
            verification_hash: hash_hex(bincode::serialize(manifest).expect("serializable")),
        }
    }
}
