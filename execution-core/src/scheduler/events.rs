use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledEvent {
    pub sequence: u64,
    pub source: String,
    pub payload: Vec<u8>,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sequence
            .cmp(&other.sequence)
            .then_with(|| self.source.cmp(&other.source))
            .then_with(|| self.payload.cmp(&other.payload))
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
