#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalityState {
    Pending,
    Replaying,
    Challenged,
    Accepted,
    Finalized,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct FinalityTracker {
    pub state: FinalityState,
    pub accepted_at_block: Option<u64>,
}

impl FinalityTracker {
    pub fn new() -> Self {
        Self {
            state: FinalityState::Pending,
            accepted_at_block: None,
        }
    }

    pub fn begin_replay(&mut self) {
        self.state = FinalityState::Replaying;
    }

    pub fn challenge(&mut self) {
        self.state = FinalityState::Challenged;
    }

    pub fn accept(&mut self, block: u64) {
        self.state = FinalityState::Accepted;
        self.accepted_at_block = Some(block);
    }

    pub fn reject(&mut self) {
        self.state = FinalityState::Rejected;
    }

    pub fn finalize_if_window_elapsed(
        &mut self,
        now_block: u64,
        challenge_window_blocks: u64,
    ) -> bool {
        match self.accepted_at_block {
            Some(accepted_at)
                if now_block.saturating_sub(accepted_at) >= challenge_window_blocks =>
            {
                self.state = FinalityState::Finalized;
                true
            }
            _ => false,
        }
    }
}
