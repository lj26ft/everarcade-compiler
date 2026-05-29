pub fn preview_asset(asset_hash: &str) -> String { crate::stable_hash(&["asset-preview", asset_hash, "projection-only"] ) }
pub fn package_membership(asset_hash: &str, package_hash: &str) -> String { crate::stable_hash(&["package-membership", asset_hash, package_hash]) }
