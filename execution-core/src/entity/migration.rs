#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MigrationRecord {
    pub from_runtime: String,
    pub to_runtime: String,
    pub node_migration: bool,
    pub verifier_migration: bool,
    pub proof_migration: bool,
}
