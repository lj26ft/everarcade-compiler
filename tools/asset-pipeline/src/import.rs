use crate::asset_pipeline::{hash, manifest};

pub fn import_asset(asset_id: &str, bytes: &[u8]) -> manifest::AssetManifest {
    manifest::canonicalize_manifest(asset_id, &hash::asset_hash(bytes))
}
