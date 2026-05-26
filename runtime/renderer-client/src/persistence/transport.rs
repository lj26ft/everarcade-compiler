use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::hash;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionTransportEnvelope {
    pub envelope_id: String,
    pub session_id: String,
    pub sequence: u64,
    pub payload_hash: String,
    pub continuity_hash: String,
}

#[derive(Default)]
pub struct ProjectionTransportIngestor {
    seen: HashSet<String>,
    pub last_sequence: Option<u64>,
    pub last_continuity_hash: Option<String>,
}

impl ProjectionTransportIngestor {
    pub fn ingest(&mut self, envelope: &ProjectionTransportEnvelope) -> Result<(), String> {
        if !self.seen.insert(envelope.envelope_id.clone()) {
            return Err("duplicate envelope".into());
        }
        if let Some(last) = self.last_sequence {
            if envelope.sequence != last + 1 {
                return Err("transport ordering violation".into());
            }
        }
        self.last_sequence = Some(envelope.sequence);
        self.last_continuity_hash = Some(envelope.continuity_hash.clone());
        Ok(())
    }

    pub fn continuity_for(
        previous: Option<&ProjectionTransportEnvelope>,
        payload_hash: &str,
    ) -> String {
        let prev = previous
            .map(|p| p.continuity_hash.as_str())
            .unwrap_or("genesis");
        hash::stable_hash(&format!("{prev}:{payload_hash}"))
    }
}
