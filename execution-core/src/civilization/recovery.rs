use super::{runtime::CivilizationRuntimeState, validation::validate_civilization_equivalence};
pub fn restore_civilization(
    checkpoint: &CivilizationRuntimeState,
    replay: &CivilizationRuntimeState,
) -> Result<CivilizationRuntimeState, &'static str> {
    validate_civilization_equivalence(checkpoint, replay)?;
    Ok(replay.clone())
}
