use super::{
    civilization_package::CivilizationPackage,
    civilization_root::compute_civilization_root,
    genesis::{CivilizationGenesis, Hash},
};

fn hash_combine(parts: &[&[u8]]) -> Hash {
    let mut out = [0u8; 32];
    for (i, p) in parts.iter().enumerate() {
        for (j, b) in p.iter().enumerate() {
            out[(i + j) % 32] ^= b.wrapping_add((i as u8).wrapping_mul(17));
        }
    }
    out
}

fn derive(stage: &[u8], root: &Hash) -> Hash {
    hash_combine(&[stage, root])
}

pub fn execute_civilization_genesis_flow(genesis: CivilizationGenesis) -> CivilizationPackage {
    let execution_root = compute_civilization_root(&genesis);
    let receipt_root = derive(b"receipt", &execution_root);
    let replay_root = derive(b"replay", &receipt_root);
    let proof_root = derive(b"proof", &replay_root);
    let checkpoint_root = derive(b"checkpoint", &proof_root);

    CivilizationPackage {
        genesis,
        execution_root,
        replay_root,
        proof_root,
        checkpoint_root,
    }
}
