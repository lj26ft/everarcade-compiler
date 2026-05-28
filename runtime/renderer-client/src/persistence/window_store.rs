use crate::transport_runtime::wire::ReplayWindowWireMessage;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Default)]
pub struct LiveReplayWindowStore {
    pub windows: BTreeMap<u64, ReplayWindowWireMessage>,
}
impl LiveReplayWindowStore {
    pub fn append(&mut self, window: ReplayWindowWireMessage) -> Result<(), String> {
        window.validate()?;
        if self.windows.contains_key(&window.start_sequence) {
            return Err("duplicate_replay_window".into());
        }
        self.windows.insert(window.start_sequence, window);
        Ok(())
    }
    pub fn compact_safe(&mut self, retain_from: u64) {
        self.windows.retain(|_, w| w.end_sequence >= retain_from);
    }
}
