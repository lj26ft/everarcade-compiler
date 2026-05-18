use serde::{Deserialize, Serialize};

use super::{advertisement::ContinuityAdvertisement, window::SyncWindow};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PushAnnouncement {
    pub advertisement: ContinuityAdvertisement,
    pub window: SyncWindow,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PushVerificationReport {
    pub continuity_ok: bool,
    pub replay_ok: bool,
    pub lineage_ok: bool,
}
