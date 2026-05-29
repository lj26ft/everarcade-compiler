use crate::stable_hash;

pub fn reload_asset_hash(asset_id: &str, content_hash: &str) -> String { stable_hash(&["hot-reload-asset", asset_id, content_hash]) }
