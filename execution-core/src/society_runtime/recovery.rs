use super::{runtime::SocietyRuntimeState, validation::equivalent};

pub fn restore(
    checkpoint: &SocietyRuntimeState,
    replay: &SocietyRuntimeState,
) -> Result<SocietyRuntimeState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
