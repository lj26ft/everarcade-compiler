use crate::{diagnostic, editor, CreatorDiagnostic};

pub fn validate_editor_surface() -> CreatorDiagnostic { diagnostic("editor-validation", &["replay-safe", "no-authority-bypass", "no-hidden-mutation"] ) }

pub fn editor_replay_equivalence(frames: &[&str]) -> bool {
    let a = editor::replay::inspect_replay(frames);
    let b = editor::replay::inspect_replay(frames);
    a == b && a.reconstruction_only
}
