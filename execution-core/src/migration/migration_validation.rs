use super::sovereign_migration::SovereignMigration;

pub fn validate_migration(m: &SovereignMigration) -> bool {
    m.source_domain != m.target_domain
}
