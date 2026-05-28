use super::{runtime::FactionRuntimeState, validation::equivalent};

pub fn restore(
    checkpoint: &FactionRuntimeState,
    replay: &FactionRuntimeState,
) -> Result<FactionRuntimeState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
