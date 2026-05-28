#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameplayPlayer {
    pub player_id: String,
    pub authority_token: String,
}

impl GameplayPlayer {
    pub fn new(player_id: impl Into<String>, session_id: &str) -> Self {
        let player_id = player_id.into();
        Self {
            authority_token: format!("auth:{session_id}:{player_id}"),
            player_id,
        }
    }
}
