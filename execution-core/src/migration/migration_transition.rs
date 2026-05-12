use super::sovereign_migration::SovereignMigration;

pub fn transition_target(m: &SovereignMigration, target_domain: [u8; 32]) -> SovereignMigration {
    let mut next = m.clone();
    next.target_domain = target_domain;
    next
}
