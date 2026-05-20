use super::event::WorldEvent;

#[derive(Debug, Clone, Default)]
pub struct WorldEventQueue {
    pub pending: Vec<WorldEvent>,
}

impl WorldEventQueue {
    pub fn push(&mut self, event: WorldEvent) {
        self.pending.push(event);
        self.pending.sort_by_key(|e| e.sequence);
    }

    pub fn pop_due(&mut self, upto_sequence: u64) -> Vec<WorldEvent> {
        let idx = self
            .pending
            .partition_point(|e| e.sequence <= upto_sequence);
        self.pending.drain(0..idx).collect()
    }
}
