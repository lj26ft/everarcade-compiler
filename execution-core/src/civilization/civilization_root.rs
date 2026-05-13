use super::genesis::{CivilizationGenesis, Hash};

fn hash_combine(parts: &[&[u8]]) -> Hash {
    let mut out = [0u8; 32];
    for (i, p) in parts.iter().enumerate() {
        for (j, b) in p.iter().enumerate() {
            out[(i + j) % 32] = out[(i + j) % 32].wrapping_add(*b).rotate_left(1);
        }
    }
    out
}

pub fn compute_civilization_root(genesis: &CivilizationGenesis) -> Hash {
    hash_combine(&[
        &genesis.civilization_id,
        &genesis.domain_root,
        &genesis.constitution_root,
        &genesis.treasury_root,
        &genesis.fiscal_root,
        &genesis.monetary_root,
        &genesis.asset_root,
    ])
}
