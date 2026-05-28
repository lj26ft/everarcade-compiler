use super::{runtime::PersistentMultiplayerRuntime, validation::validate_multiplayer};
pub fn restore_multiplayer(
    m: &PersistentMultiplayerRuntime,
) -> Result<PersistentMultiplayerRuntime, &'static str> {
    if validate_multiplayer(m) {
        Ok(m.clone())
    } else {
        Err("multiplayer continuity divergence rejected")
    }
}
