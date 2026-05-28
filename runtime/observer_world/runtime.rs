#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObserverWorldRuntime {
    pub hydrated_root: String,
    pub reconstruction_only: bool,
}

impl ObserverWorldRuntime {
    pub fn hydrate(replay_tip: &str) -> Self {
        Self {
            hydrated_root: format!("observer-world:hydrated:{replay_tip}"),
            reconstruction_only: true,
        }
    }
}
