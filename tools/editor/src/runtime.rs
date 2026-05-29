use crate::{diagnostic, reject_authority_bypass, CreatorDiagnostic};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeView { pub tick: u64, pub state_root: String, pub checkpoint: String }

pub fn visualize_runtime_state(tick: u64, state_root: &str) -> RuntimeView {
    RuntimeView { tick, state_root: state_root.to_owned(), checkpoint: format!("checkpoint-{tick:016}") }
}

pub fn runtime_editor_diagnostic() -> CreatorDiagnostic { diagnostic("visual-runtime-editor", &["runtime", "world", "session", "replay"] ) }

pub fn request_authority_bypass(requested: bool) -> Result<(), &'static str> { reject_authority_bypass(requested) }
