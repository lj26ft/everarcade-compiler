use crate::persistence::{
    archive::WorldArchive, compression::ReplayCompressionManifest, storage_lineage::StorageLineage,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorationManifest {
    pub archive_hash: String,
    pub compression_hash: String,
    pub storage_lineage_hash: String,
    pub continuity_root: String,
}

pub fn restore_continuity(
    archive: &WorldArchive,
    compression: &ReplayCompressionManifest,
    lineage: &StorageLineage,
) -> Result<RestorationManifest, String> {
    let archive_hash = archive.canonical_hash()?;
    let compression_hash = compression.canonical_hash()?;
    let storage_lineage_hash = lineage.canonical_hash()?;
    let continuity_root = crate::hashing::hash_bytes(
        format!("{archive_hash}|{compression_hash}|{storage_lineage_hash}").as_bytes(),
    );
    Ok(RestorationManifest {
        archive_hash,
        compression_hash,
        storage_lineage_hash,
        continuity_root,
    })
}
