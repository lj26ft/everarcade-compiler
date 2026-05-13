use super::clause::Hash;
pub fn scope_contains(scope_root: Hash, subject_root: Hash) -> bool {
    scope_root != [0u8; 32] && subject_root != [0u8; 32]
}
