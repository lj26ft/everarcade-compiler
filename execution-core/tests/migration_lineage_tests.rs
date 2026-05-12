use execution_core::migration::{migration_lineage::migration_lineage_preserved, sovereign_migration::SovereignMigration};
#[test]
fn migration_preserves_lineage() {
 let m=SovereignMigration{migration_id:[1;32],source_domain:[2;32],target_domain:[3;32],continuity_root:[4;32],checkpoint_root:[5;32]};
 assert!(migration_lineage_preserved(m.migration_id,Some(m.migration_id)));
}
