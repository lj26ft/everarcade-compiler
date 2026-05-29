use super::editor::EditingEngine;

pub fn interactive_editing_equivalence() -> bool {
    let mut first = EditingEngine::sample();
    let mut second = EditingEngine::sample();
    first.select(&["entity:settler", "resource:crystal"]);
    second.select(&["resource:crystal", "entity:settler"]);
    first.group_selection("group:starter");
    second.group_selection("group:starter");
    first.copy().ok() == second.copy().ok()
        && first.paste("copy").ok() == second.paste("copy").ok()
        && first.duplicate("dupe").ok() == second.duplicate("dupe").ok()
        && first.delete().ok() == second.delete().ok()
        && first.history.browser_entries() == second.history.browser_entries()
        && first.request_authority_mutation(true).is_err()
        && first.history.undo().is_some()
        && first.history.redo().is_some()
}

pub fn undo_redo_equivalence() -> bool {
    let mut engine = EditingEngine::sample();
    engine.select(&["entity:settler"]);
    engine.copy().unwrap();
    let before = engine.history.history_hash.clone();
    let undone = engine.undo().is_some();
    let redone = engine.redo().is_some();
    undone && redone && engine.history.history_hash == before
}

pub fn replay_safe_editor_behavior() -> bool {
    let engine = EditingEngine::sample();
    engine.replay_safe
        && engine.runtime_authority_respected
        && engine.request_authority_mutation(true).is_err()
}
