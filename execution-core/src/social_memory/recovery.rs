use super::{runtime::SocialMemoryState, validation::equivalent};

pub fn restore(
    checkpoint: &SocialMemoryState,
    replay: &SocialMemoryState,
) -> Result<SocialMemoryState, &'static str> {
    equivalent(checkpoint, replay)?;
    Ok(replay.clone())
}
