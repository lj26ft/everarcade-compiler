#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayInput {
    pub player_id: String,
    pub frame: u64,
    pub delta: u64,
    pub authority_token: String,
}

impl GameplayInput {
    pub fn new(
        player_id: impl Into<String>,
        frame: u64,
        delta: u64,
        authority_token: impl Into<String>,
    ) -> Self {
        Self {
            player_id: player_id.into(),
            frame,
            delta,
            authority_token: authority_token.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthorityBoundary {
    DeterministicRuntime,
    ReplayObserver,
    Renderer,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayExecution {
    pub boundary: AuthorityBoundary,
    pub scheduled_tick: u64,
    pub input: GameplayInput,
}
