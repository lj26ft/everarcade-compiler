#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecutionFrame {
    pub tick: u64,
    pub player_id: String,
    pub input_delta: u64,
}
