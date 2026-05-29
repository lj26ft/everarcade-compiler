use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorSession { pub session_id: String, pub continuity_root: String, pub hidden_mutation_allowed: bool }

pub fn open_session(session_id: &str, replay_root: &str) -> EditorSession {
    EditorSession { session_id: session_id.to_owned(), continuity_root: stable_hash(&[session_id, replay_root]), hidden_mutation_allowed: false }
}
