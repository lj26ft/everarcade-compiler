#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ObserverStream;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LiveObserverReplayWindow {
    pub start_sequence: u64,
    pub end_sequence: u64,
    pub continuity_root: String,
    pub reconstruction_only: bool,
}

impl LiveObserverReplayWindow {
    pub fn validate(&self) -> Result<(), String> {
        if !self.reconstruction_only {
            return Err("observer_authority_mutation_rejected".into());
        }
        if self.continuity_root.is_empty() {
            return Err("observer_continuity_root_rejected".into());
        }
        if self.end_sequence < self.start_sequence {
            return Err("observer_window_bounds_rejected".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct LiveObserverReplayStream {
    pub cursor: u64,
    pub windows: Vec<LiveObserverReplayWindow>,
}
impl LiveObserverReplayStream {
    pub fn consume(&mut self, window: LiveObserverReplayWindow) -> Result<(), String> {
        window.validate()?;
        if window.start_sequence != self.cursor {
            return Err("observer_replay_order_rejected".into());
        }
        self.cursor = window.end_sequence + 1;
        self.windows.push(window);
        Ok(())
    }
}
