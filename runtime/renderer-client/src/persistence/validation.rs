use super::{artifact::RenderProjectionArtifact, manifest::ProjectionArtifactManifest};

pub fn validate_projection_continuity(frames: &[RenderProjectionArtifact]) -> Result<(), String> {
    for (idx, frame) in frames.iter().enumerate() {
        if frame.frame_index != idx as u64 {
            return Err("frame ordering divergence".into());
        }
    }
    Ok(())
}

pub fn validate_manifest_integrity(manifest: &ProjectionArtifactManifest) -> Result<(), String> {
    if manifest.artifact_count != manifest.artifact_hashes.len() as u64 {
        return Err("manifest count mismatch".into());
    }
    Ok(())
}

pub fn validate_replay_equivalence(original: &[RenderProjectionArtifact], replayed: &[RenderProjectionArtifact]) -> Result<(), String> {
    if original == replayed { Ok(()) } else { Err("replay mismatch".into()) }
}
