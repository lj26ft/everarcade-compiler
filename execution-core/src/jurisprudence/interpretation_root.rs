use super::precedent::Hash;
pub fn compute_interpretation_root(constitutional_root: Hash, doctrine_root: Hash) -> Hash {
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = constitutional_root[i] ^ doctrine_root[i].rotate_left(1);
    }
    out
}
