use serde::{Deserialize, Serialize};

use super::bundle::ContinuityBundle;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FederationPersistedState {
    pub peer_sessions: Vec<String>,
    pub topology_state: Vec<u8>,
    pub lease_state: Vec<u8>,
    pub sync_checkpoint: [u8; 32],
    pub transport_continuity: Vec<u8>,
    pub bundle: ContinuityBundle,
}

pub fn export_federation_state(state: &FederationPersistedState) -> Result<Vec<u8>, String> {
    serde_json::to_vec(state).map_err(|e| e.to_string())
}

pub fn import_federation_state(bytes: &[u8]) -> Result<FederationPersistedState, String> {
    serde_json::from_slice(bytes).map_err(|e| e.to_string())
}

pub fn restore_distributed_continuity(bytes: &[u8]) -> Result<ContinuityBundle, String> {
    Ok(import_federation_state(bytes)?.bundle)
}
