use crate::stable_hash;

pub fn inspect_session_lineage(session_id: &str, replay_root: &str) -> String { stable_hash(&["entity-session", session_id, replay_root]) }
