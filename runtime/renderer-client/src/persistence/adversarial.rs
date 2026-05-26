use std::collections::HashSet;

use super::{artifact::RenderProjectionArtifact, transport::ProjectionTransportEnvelope};

pub fn detect_frame_ordering_corruption(
    artifacts: &[RenderProjectionArtifact],
) -> Result<(), String> {
    for (idx, item) in artifacts.iter().enumerate() {
        if item.frame_index != idx as u64 {
            return Err("frame ordering corruption".into());
        }
    }
    Ok(())
}

pub fn detect_duplicate_artifact_insertion(
    artifacts: &[RenderProjectionArtifact],
) -> Result<(), String> {
    let mut seen = HashSet::new();
    for artifact in artifacts {
        if !seen.insert(artifact.artifact_id.as_str()) {
            return Err("duplicate artifact insertion".into());
        }
    }
    Ok(())
}

pub fn detect_transport_replay_injection(
    envelopes: &[ProjectionTransportEnvelope],
) -> Result<(), String> {
    let mut seen = HashSet::new();
    for e in envelopes {
        if !seen.insert(e.envelope_id.as_str()) {
            return Err("transport envelope replay injection".into());
        }
    }
    Ok(())
}
