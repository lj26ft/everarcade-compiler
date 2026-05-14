use super::archive_package::Hash;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArchiveManifest {
    pub archive_root: Hash,
    pub epoch_start: u64,
    pub epoch_end: u64,
}
