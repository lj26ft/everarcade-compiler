use crate::{diagnostic, entity_inspector, reject_authority_bypass, CreatorDiagnostic};

pub fn validate_entity_inspector() -> CreatorDiagnostic { diagnostic("entity-inspector-validation", &["read-only", "replay-safe", "authority-visible"] ) }

pub fn entity_inspector_replay_safety(entity_id: &str, components: &[&str]) -> bool { entity_inspector::entity::inspect_entity(entity_id, components) == entity_inspector::entity::inspect_entity(entity_id, components) }

pub fn request_authority_bypass(requested: bool) -> Result<(), &'static str> { reject_authority_bypass(requested) }
