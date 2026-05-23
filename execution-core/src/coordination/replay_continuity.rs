use crate::coordination::epoch::FederationEpochManifest;
use crate::coordination::migration::MigrationJournal;

pub fn validate_replay_lineage(history: &[FederationEpochManifest]) -> bool {
    history
        .windows(2)
        .all(|w| w[0].continuity_root == w[1].continuity_root)
}

pub fn validate_checkpoint_continuity(history: &[FederationEpochManifest]) -> bool {
    history
        .windows(2)
        .all(|w| !w[0].checkpoint_hash.is_empty() && !w[1].checkpoint_hash.is_empty())
}

pub fn validate_migration_continuity(journal: &MigrationJournal) -> bool {
    journal.upgrades.len() == journal.transitions.len()
}

pub fn validate_upgrade_safe_replay(
    history: &[FederationEpochManifest],
    journal: &MigrationJournal,
) -> Result<(), &'static str> {
    if !validate_replay_lineage(history) {
        return Err("incompatible replay lineage");
    }
    if !validate_checkpoint_continuity(history) {
        return Err("incompatible checkpoint continuity");
    }
    if !validate_migration_continuity(journal) {
        return Err("incompatible migration continuity");
    }
    Ok(())
}
