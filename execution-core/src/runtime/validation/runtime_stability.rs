#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeStabilityWindow {
    pub checkpoint_lineage_preserved: bool,
    pub continuity_preserved: bool,
    pub replay_seek_stable: bool,
}

impl RuntimeStabilityWindow {
    pub fn is_stable(&self) -> bool {
        self.checkpoint_lineage_preserved && self.continuity_preserved && self.replay_seek_stable
    }
}
