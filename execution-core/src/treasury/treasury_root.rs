use super::treasury::Hash;

pub fn derive_treasury_root(monetary_root: Hash, fiscal_root: Hash) -> Hash {
    let mut out = [0u8; 32];
    for i in 0..32 { out[i] = monetary_root[i].wrapping_add(fiscal_root[i]); }
    out
}
