use super::treasury::Hash;

pub fn derive_treasury_id(sovereign_domain: Hash, constitutional_root: Hash) -> Hash {
    let mut out = [0u8; 32];
    for i in 0..32 { out[i] = sovereign_domain[i] ^ constitutional_root[i]; }
    out
}
