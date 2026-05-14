pub type Hash = [u8; 32];
pub fn derive_asset_lineage(prior: Hash, transition: Hash) -> Hash {
    let mut o = [0; 32];
    for i in 0..32 {
        o[i] = prior[i] ^ transition[i];
    }
    o
}
