use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayTimeline {
    pub frames: Vec<String>,
    pub checkpoints: Vec<String>,
    pub divergence_markers: Vec<String>,
    pub continuity_hash: String,
    pub cursor: usize,
    pub reconstruction_only: bool,
}

impl ReplayTimeline {
    pub fn sample() -> Self {
        Self::new(
            vec![
                "frame-0001".into(),
                "frame-0002".into(),
                "frame-0003".into(),
            ],
            vec!["checkpoint-a".into(), "checkpoint-b".into()],
            vec!["none".into()],
        )
    }

    pub fn new(
        mut frames: Vec<String>,
        mut checkpoints: Vec<String>,
        mut divergence_markers: Vec<String>,
    ) -> Self {
        frames.sort();
        checkpoints.sort();
        divergence_markers.sort();
        let mut parts = vec!["replay-ui".to_owned(), "reconstruction-only".to_owned()];
        parts.extend(frames.iter().cloned());
        parts.extend(
            checkpoints
                .iter()
                .map(|checkpoint| format!("checkpoint:{checkpoint}")),
        );
        parts.extend(
            divergence_markers
                .iter()
                .map(|marker| format!("divergence:{marker}")),
        );
        Self {
            frames,
            checkpoints,
            divergence_markers,
            continuity_hash: stable_hash(&parts.iter().map(String::as_str).collect::<Vec<_>>()),
            cursor: 0,
            reconstruction_only: true,
        }
    }

    pub fn scrub(&mut self, cursor: usize) -> Option<&str> {
        self.cursor = cursor.min(self.frames.len().saturating_sub(1));
        self.frames.get(self.cursor).map(String::as_str)
    }

    pub fn request_replay_mutation(&self, requested: bool) -> Result<(), &'static str> {
        if requested {
            Err("replay timeline is reconstruction-only")
        } else {
            Ok(())
        }
    }
}
