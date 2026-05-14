use super::archive_package::Hash;
use super::archive_root::hash_roots;
pub fn replay_summary_root(replay_roots: &[Hash]) -> Hash {
    hash_roots(replay_roots)
}
