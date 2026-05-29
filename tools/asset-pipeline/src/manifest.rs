use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetManifest { pub asset_id: String, pub content_hash: String, pub canonical_manifest_hash: String }

pub fn canonicalize_manifest(asset_id: &str, content_hash: &str) -> AssetManifest {
    AssetManifest { asset_id: asset_id.to_owned(), content_hash: content_hash.to_owned(), canonical_manifest_hash: stable_hash(&[asset_id, content_hash]) }
}
