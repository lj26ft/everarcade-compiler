use super::hash_hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageLayerProof {
    pub layer_name: String,
    pub layer_hash: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeImageManifest {
    pub config_hash: String,
    pub deployment_manifest_hash: String,
    pub operational_manifest_hash: String,
    pub continuity_metadata_hash: String,
    pub release_lineage_hash: String,
    pub manifest_root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageGenerationReceipt {
    pub image_hash: String,
    pub manifest_root: String,
}

pub struct RuntimeImageBuilder;
impl RuntimeImageBuilder {
    pub fn build(layers: Vec<ImageLayerProof>) -> (RuntimeImageManifest, ImageGenerationReceipt) {
        let mut ordered = layers;
        ordered.sort_by(|a, b| a.layer_name.cmp(&b.layer_name));
        let root = hash_hex(bincode::serialize(&ordered).expect("serializable"));
        let manifest = RuntimeImageManifest {
            config_hash: hash_hex("runtime-config"),
            deployment_manifest_hash: hash_hex("deployment"),
            operational_manifest_hash: hash_hex("operational"),
            continuity_metadata_hash: hash_hex("continuity"),
            release_lineage_hash: hash_hex("release-lineage"),
            manifest_root: root.clone(),
        };
        let image_hash = hash_hex(bincode::serialize(&manifest).expect("serializable"));
        let receipt = ImageGenerationReceipt {
            image_hash: image_hash.clone(),
            manifest_root: root,
        };
        (manifest, receipt)
    }
}
