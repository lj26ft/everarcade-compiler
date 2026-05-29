use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvertedAsset { pub format: String, pub output_hash: String }

pub fn convert_asset(format: &str, source_hash: &str) -> ConvertedAsset { ConvertedAsset { format: format.to_owned(), output_hash: stable_hash(&["convert", format, source_hash]) } }
