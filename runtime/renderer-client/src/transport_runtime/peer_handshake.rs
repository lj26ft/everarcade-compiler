use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayPeerHandshake {
    pub peer_id: String,
    pub protocol_version: String,
    pub replay_tip: u64,
    pub continuity_root: String,
    pub supported_transports: Vec<String>,
    pub reconstruction_only: bool,
}

impl ReplayPeerHandshake {
    pub fn validate_against(
        &self,
        expected_protocol: &str,
        expected_root: &str,
    ) -> Result<(), String> {
        if self.peer_id.trim().is_empty() {
            return Err("peer_identity_rejected".into());
        }
        if !self.reconstruction_only {
            return Err("authority_mutation_rejected".into());
        }
        if self.protocol_version != expected_protocol {
            return Err("protocol_incompatibility_rejected".into());
        }
        if self.continuity_root != expected_root {
            return Err("continuity_root_divergence_rejected".into());
        }
        if self.supported_transports.is_empty() {
            return Err("peer_transport_capability_rejected".into());
        }
        Ok(())
    }
}
