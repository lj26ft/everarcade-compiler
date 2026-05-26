#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReplayWindow {
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReplayWindowCursor {
    pub position: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationReplayWindowResult {
    pub verified: bool,
    pub last_position: u64,
}
