use super::precedent::Hash;
pub fn derive_doctrine_root(precedent_roots: &[Hash]) -> Hash {
    let mut out = [0u8; 32];
    for root in precedent_roots { for i in 0..32 { out[i] = out[i].wrapping_add(root[i]); } }
    out
}
