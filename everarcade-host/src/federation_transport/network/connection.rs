#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectionState {
    pub peer: String,
    pub connected: bool,
    pub last_sequence: u64,
}
