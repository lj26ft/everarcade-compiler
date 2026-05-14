use super::asset_constitution::AssetConstitution;
pub fn validate_asset(asset: &AssetConstitution) -> bool {
    asset.asset_id != [0u8; 32]
}
