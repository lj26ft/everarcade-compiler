use crate::{asset_pipeline, diagnostic, CreatorDiagnostic};

pub fn validate_asset_compatibility(asset_type: &str) -> Result<CreatorDiagnostic, &'static str> {
    match asset_type { "image" | "audio" | "manifest" | "script-data" => Ok(diagnostic("asset-pipeline-validation", &[asset_type, "compatible"])), _ => Err("incompatible creator asset") }
}

pub fn asset_pipeline_hash_equivalence(asset_id: &str, bytes: &[u8]) -> bool { asset_pipeline::import::import_asset(asset_id, bytes) == asset_pipeline::import::import_asset(asset_id, bytes) }
