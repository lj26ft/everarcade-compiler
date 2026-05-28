//! Sovereign game manifest validation.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub tick_rate: u16,
    pub deterministic: bool,
    pub assets_manifest: String,
    pub replay_enabled: bool,
    pub checkpoint_interval: u64,
}

impl GameManifest {
    pub fn example() -> Self {
        Self {
            id: "example-game".into(),
            name: "Example Game".into(),
            version: "0.1.0".into(),
            tick_rate: 30,
            deterministic: true,
            assets_manifest: "assets.toml".into(),
            replay_enabled: true,
            checkpoint_interval: 300,
        }
    }
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.id.is_empty() || self.name.is_empty() || self.version.is_empty() {
            return Err("game identity is required");
        }
        if self.tick_rate == 0 || self.tick_rate > 240 {
            return Err("invalid deterministic tick rate");
        }
        if !self.deterministic {
            return Err("runtime must be deterministic");
        }
        if !self.replay_enabled {
            return Err("replay must be enabled");
        }
        if self.checkpoint_interval == 0 {
            return Err("checkpoint interval is required");
        }
        Ok(())
    }
}
