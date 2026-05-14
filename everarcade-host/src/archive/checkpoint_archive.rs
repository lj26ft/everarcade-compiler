use super::archive_package::Hash;
use super::archive_root::hash_roots;
pub fn checkpoint_archive_root(checkpoint_roots: &[Hash]) -> Hash { hash_roots(checkpoint_roots) }
