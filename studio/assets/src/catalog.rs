use crate::assets::import::ImportedAsset;
pub fn catalog_hash(assets: &[ImportedAsset]) -> String { let mut rows: Vec<String> = assets.iter().map(|a| format!("{}:{}", a.asset_id, a.asset_hash)).collect(); rows.sort(); let mut parts = vec!["asset-catalog"]; parts.extend(rows.iter().map(String::as_str)); crate::stable_hash(&parts) }
