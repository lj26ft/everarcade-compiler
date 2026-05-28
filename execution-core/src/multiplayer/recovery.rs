use super::runtime::MultiplayerRuntime;
pub fn recover_multiplayer(
    continuity_root: impl Into<String>,
    next_frame: u64,
) -> MultiplayerRuntime {
    let mut r = MultiplayerRuntime::new(continuity_root);
    r.frame = next_frame;
    r
}
