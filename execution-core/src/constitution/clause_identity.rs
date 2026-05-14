use super::clause::Hash;
pub fn clause_identity(scope_root: Hash, execution_root: Hash) -> Hash {
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = scope_root[i] ^ execution_root[i] ^ (i as u8);
    }
    out
}
