#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportedAsset { pub asset_id: String, pub asset_hash: String, pub compatible: bool }
pub fn import_asset(asset_id: &str, bytes: &[u8], kind: &str) -> ImportedAsset { let hex = crate::stable_hash(&["asset-bytes", &hex::encode(bytes)]); ImportedAsset { asset_id: asset_id.to_owned(), asset_hash: crate::stable_hash(&["import-asset", asset_id, &hex, kind]), compatible: kind != "wall-clock-plugin" } }
