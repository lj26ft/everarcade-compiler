#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRetentionRuntime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRetentionPolicy {
    pub min_window: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRetentionManifest {
    pub retained: Vec<String>,
}

impl ArtifactRetentionRuntime {
    pub fn apply(
        policy: &ArtifactRetentionPolicy,
        artifacts: &[String],
    ) -> Result<ArtifactRetentionManifest, String> {
        if artifacts.len() < policy.min_window {
            return Err("invalid retention truncation".into());
        }
        Ok(ArtifactRetentionManifest {
            retained: artifacts[artifacts.len() - policy.min_window..].to_vec(),
        })
    }
}
