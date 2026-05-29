#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioSession {
    pub session_hash: String,
    pub workflow: Vec<&'static str>,
    pub replay_safe: bool,
}

pub const CREATOR_WORKFLOW: &[&str] = &[
    "Create Project", "Import Assets", "Build World", "Place Entities", "Configure Runtime",
    "Run Simulation", "Inspect Replay", "Debug Divergence", "Package Content", "Deploy Runtime",
];

pub fn start_session(project_id: &str) -> StudioSession {
    let mut parts = vec!["studio-session", project_id];
    parts.extend(CREATOR_WORKFLOW.iter().copied());
    StudioSession { session_hash: crate::stable_hash(&parts), workflow: CREATOR_WORKFLOW.to_vec(), replay_safe: true }
}
