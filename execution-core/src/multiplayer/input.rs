#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerInput {
    pub player_id: String,
    pub frame: u64,
    pub delta: u64,
}
