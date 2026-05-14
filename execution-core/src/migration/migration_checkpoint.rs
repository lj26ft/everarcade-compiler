use super::sovereign_migration::SovereignMigration;

pub fn preserves_checkpoint(m: &SovereignMigration, expected: [u8; 32]) -> bool {
    m.checkpoint_root == expected
}
