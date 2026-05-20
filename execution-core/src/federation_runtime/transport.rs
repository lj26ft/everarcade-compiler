use serde::{Deserialize, Serialize};

use super::bundle::ContinuityBundle;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FramedContinuityMessage {
    pub sequence: u64,
    pub payload: ContinuityBundle,
}

pub fn frame_message(sequence: u64, payload: ContinuityBundle) -> FramedContinuityMessage {
    FramedContinuityMessage { sequence, payload }
}
