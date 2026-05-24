use serde::{Deserialize, Serialize};

use super::hash_hex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleArtifact {
    pub name: String,
    pub hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleProof {
    pub bundle_hash: String,
    pub artifact_root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleGenerationReceipt {
    pub bundle_hash: String,
    pub proof: BundleProof,
    pub archive_path: String,
}

pub struct DeploymentBundleBuilder;
impl DeploymentBundleBuilder {
    pub fn generate(
        mut artifacts: Vec<(String, Vec<u8>)>,
        archive_path: &str,
    ) -> (Vec<BundleArtifact>, BundleGenerationReceipt) {
        artifacts.sort_by(|a, b| a.0.cmp(&b.0));
        let out: Vec<BundleArtifact> = artifacts
            .into_iter()
            .map(|(n, b)| BundleArtifact {
                name: n,
                hash: hash_hex(b),
            })
            .collect();
        let artifact_root = hash_hex(bincode::serialize(&out).expect("serializable"));
        let bundle_hash = hash_hex(format!("bundle:{}", artifact_root));
        std::fs::write(
            archive_path,
            bincode::serialize(&(bundle_hash.clone(), &out)).expect("serializable"),
        )
        .expect("write archive");
        (
            out,
            BundleGenerationReceipt {
                bundle_hash: bundle_hash.clone(),
                proof: BundleProof {
                    bundle_hash,
                    artifact_root,
                },
                archive_path: archive_path.to_string(),
            },
        )
    }
}
