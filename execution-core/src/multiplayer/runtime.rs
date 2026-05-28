use super::{
    coordination::deterministic_order, frame_sync::FrameSync, input::PlayerInput,
    validation::MultiplayerError,
};
#[derive(Clone, Debug)]
pub struct MultiplayerRuntime {
    pub continuity_root: String,
    pub frame: u64,
    pub syncs: Vec<FrameSync>,
}
impl MultiplayerRuntime {
    pub fn new(continuity_root: impl Into<String>) -> Self {
        Self {
            continuity_root: continuity_root.into(),
            frame: 1,
            syncs: Vec::new(),
        }
    }
    pub fn synchronize(
        &mut self,
        inputs: Vec<PlayerInput>,
    ) -> Result<Vec<PlayerInput>, MultiplayerError> {
        let ordered = deterministic_order(inputs, self.frame)?;
        let ordered_players = ordered.iter().map(|i| i.player_id.clone()).collect();
        self.syncs.push(FrameSync {
            frame: self.frame,
            ordered_players,
            continuity_root: self.continuity_root.clone(),
        });
        self.frame += 1;
        Ok(ordered)
    }
}
