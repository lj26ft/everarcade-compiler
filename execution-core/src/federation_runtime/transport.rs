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

pub fn recover_peer_connection(connected: &mut bool) {
    *connected = true;
}

pub fn recover_federation_continuity(
    connected: bool,
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Option<ContinuityBundle> {
    if connected {
        super::sync::synchronize(local, peer)
    } else {
        None
    }
}

pub fn resume_incremental_sync(
    connected: bool,
    local: &ContinuityBundle,
    peer: &ContinuityBundle,
) -> Option<ContinuityBundle> {
    if connected {
        super::sync::sync_incremental_journal(local, peer)
    } else {
        None
    }
}
