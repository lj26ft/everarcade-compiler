use super::{runtime::EcologyRuntimeState, validation::equivalent};

pub fn restore(
    checkpoint: &EcologyRuntimeState,
    replay: &EcologyRuntimeState,
) -> Result<EcologyRuntimeState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
