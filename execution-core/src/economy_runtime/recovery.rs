use super::{runtime::EconomyRuntime, validation::validate_ledger_equivalence};
pub fn restore_economy(
    checkpoint: &EconomyRuntime,
    replay: &EconomyRuntime,
) -> Result<EconomyRuntime, &'static str> {
    validate_ledger_equivalence(checkpoint, replay)?;
    Ok(replay.clone())
}
