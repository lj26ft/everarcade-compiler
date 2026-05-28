use super::{runtime::ProceduralWorldState, validation::equivalent};

pub fn restore(
    checkpoint: &ProceduralWorldState,
    replay: &ProceduralWorldState,
) -> Result<ProceduralWorldState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
