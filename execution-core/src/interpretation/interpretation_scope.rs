use super::interpretation::Hash;
pub fn scope_applies(scope: Hash, target: Hash) -> bool {
    scope != [0u8; 32] && target != [0u8; 32]
}
