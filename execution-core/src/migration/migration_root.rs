use super::sovereign_migration::{Hash, SovereignMigration};

pub fn migration_root(m: &SovereignMigration) -> Hash {
    let mut out = m.continuity_root;
    for (i, b) in m.checkpoint_root.iter().enumerate() { out[i] ^= *b; }
    out
}
