use std::collections::BTreeSet;

use super::events::ScheduledEvent;

#[derive(Clone, Debug, Default)]
pub struct DeterministicQueue {
    events: BTreeSet<ScheduledEvent>,
}

impl DeterministicQueue {
    pub fn push(&mut self, event: ScheduledEvent) -> bool {
        self.events.insert(event)
    }

    pub fn pop_next(&mut self) -> Option<ScheduledEvent> {
        let first = self.events.first().cloned()?;
        self.events.remove(&first);
        Some(first)
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
