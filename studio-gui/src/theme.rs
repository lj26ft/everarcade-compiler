#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioTheme {
    pub name: &'static str,
    pub accent: &'static str,
    pub replay_safe_color: &'static str,
}

impl Default for StudioTheme {
    fn default() -> Self {
        Self {
            name: "EverArcade Dark",
            accent: "arcade-cyan",
            replay_safe_color: "continuity-green",
        }
    }
}
