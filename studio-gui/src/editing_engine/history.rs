use super::actions::EditorActionRecord;
use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorHistory {
    pub undo_stack: Vec<EditorActionRecord>,
    pub redo_stack: Vec<EditorActionRecord>,
    pub restore_points: Vec<String>,
    pub history_hash: String,
}

impl EditorHistory {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            restore_points: Vec::new(),
            history_hash: stable_hash(&["editor-history", "empty"]),
        }
    }

    pub fn push(&mut self, action: EditorActionRecord) {
        self.undo_stack.push(action);
        self.redo_stack.clear();
        self.rebuild_hash();
    }

    pub fn undo(&mut self) -> Option<EditorActionRecord> {
        let action = self.undo_stack.pop()?;
        self.redo_stack.push(action.clone());
        self.rebuild_hash();
        Some(action)
    }

    pub fn redo(&mut self) -> Option<EditorActionRecord> {
        let action = self.redo_stack.pop()?;
        self.undo_stack.push(action.clone());
        self.rebuild_hash();
        Some(action)
    }

    pub fn restore_point(&mut self, label: &str) -> String {
        let point = stable_hash(&["restore-point", label, &self.history_hash]);
        self.restore_points.push(point.clone());
        self.rebuild_hash();
        point
    }

    pub fn browser_entries(&self) -> Vec<String> {
        self.undo_stack
            .iter()
            .map(|action| format!("{}:{}", action.sequence, action.action_type.as_str()))
            .collect()
    }

    fn rebuild_hash(&mut self) {
        let undo = self
            .undo_stack
            .iter()
            .map(|action| action.after_hash.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let redo = self
            .redo_stack
            .iter()
            .map(|action| action.after_hash.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let restore = self.restore_points.join(",");
        self.history_hash = stable_hash(&["editor-history", &undo, &redo, &restore]);
    }
}
