use super::{entity::SovereignEntity, validation::validate_entity_equivalence};
pub fn restore_entity(
    checkpoint: &SovereignEntity,
    replay: &SovereignEntity,
) -> Result<SovereignEntity, &'static str> {
    validate_entity_equivalence(checkpoint, replay)?;
    Ok(replay.clone())
}
