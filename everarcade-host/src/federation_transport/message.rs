use execution_core::federation_runtime::bundle::ContinuityBundle;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FederationMessage {
    PeerIdentity(Vec<u8>),
    Continuity(ContinuityBundle),
    Checkpoint([u8; 32]),
}
