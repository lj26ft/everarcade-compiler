use super::monetary_policy::Hash;
pub fn derive_monetary_root(issuance_root: Hash, supply_root: Hash) -> Hash {
    let mut o = [0; 32];
    for i in 0..32 {
        o[i] = issuance_root[i].wrapping_add(supply_root[i]);
    }
    o
}
