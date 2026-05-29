use super::{
    actions::{EditorActionRecord, EditorActionType},
    history::EditorHistory,
    selection::{SelectionMode, SelectionSet},
};
use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditableNode {
    pub id: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditingEngine {
    pub selection: SelectionSet,
    pub nodes: Vec<EditableNode>,
    pub clipboard: Vec<EditableNode>,
    pub history: EditorHistory,
    pub state_hash: String,
    pub replay_safe: bool,
    pub runtime_authority_respected: bool,
}

impl EditingEngine {
    pub fn sample() -> Self {
        let mut engine = Self {
            selection: SelectionSet::empty(),
            nodes: vec![
                EditableNode {
                    id: "entity:settler".into(),
                    content_hash: stable_hash(&["node", "entity:settler"]),
                },
                EditableNode {
                    id: "resource:crystal".into(),
                    content_hash: stable_hash(&["node", "resource:crystal"]),
                },
                EditableNode {
                    id: "region:north".into(),
                    content_hash: stable_hash(&["node", "region:north"]),
                },
            ],
            clipboard: Vec::new(),
            history: EditorHistory::new(),
            state_hash: String::new(),
            replay_safe: true,
            runtime_authority_respected: true,
        };
        engine.rebuild_hash();
        engine
    }

    pub fn select(&mut self, ids: &[&str]) {
        let mode = if ids.len() > 1 {
            SelectionMode::Multi
        } else {
            SelectionMode::Single
        };
        self.selection.select(ids.iter().copied(), mode);
    }

    pub fn group_selection(&mut self, group_id: &str) {
        self.selection.group(group_id);
    }

    pub fn context_actions(&self) -> Vec<&'static str> {
        let mut actions = vec!["copy", "paste"];
        if !self.selection.ids.is_empty() {
            actions.extend(["duplicate", "delete"]);
        }
        actions
    }

    pub fn copy(&mut self) -> Result<(), &'static str> {
        self.clipboard = self
            .nodes
            .iter()
            .filter(|node| self.selection.ids.contains(&node.id))
            .cloned()
            .collect();
        self.record(EditorActionType::Copy, "clipboard-copy")
    }

    pub fn paste(&mut self, suffix: &str) -> Result<(), &'static str> {
        let mut pasted = self.clipboard.clone();
        pasted.sort_by(|left, right| left.id.cmp(&right.id));
        for node in pasted {
            let id = format!("{}-{suffix}", node.id);
            let content_hash = stable_hash(&["paste", &id, &node.content_hash]);
            self.nodes.push(EditableNode { id, content_hash });
        }
        self.nodes.sort_by(|left, right| left.id.cmp(&right.id));
        self.record(EditorActionType::Paste, suffix)
    }

    pub fn duplicate(&mut self, suffix: &str) -> Result<(), &'static str> {
        self.copy()?;
        self.paste(suffix)?;
        self.record(EditorActionType::Duplicate, suffix)
    }

    pub fn delete(&mut self) -> Result<(), &'static str> {
        self.nodes
            .retain(|node| !self.selection.ids.iter().any(|id| id == &node.id));
        self.record(EditorActionType::Delete, "delete-selection")
    }

    pub fn apply_context_action(&mut self, action: &str) -> Result<(), &'static str> {
        match action {
            "copy" => self.copy(),
            "paste" => self.paste("pasted"),
            "duplicate" => self.duplicate("copy"),
            "delete" => self.delete(),
            custom => self.record(EditorActionType::Context(custom.to_owned()), custom),
        }
    }

    pub fn undo(&mut self) -> Option<EditorActionRecord> {
        self.history.undo()
    }

    pub fn redo(&mut self) -> Option<EditorActionRecord> {
        self.history.redo()
    }

    pub fn request_authority_mutation(&self, requested: bool) -> Result<(), &'static str> {
        if requested {
            Err("editing engine records deterministic editor actions instead of mutating runtime authority")
        } else {
            Ok(())
        }
    }

    fn record(&mut self, action_type: EditorActionType, payload: &str) -> Result<(), &'static str> {
        if !self.replay_safe || !self.runtime_authority_respected {
            return Err("editor action violates replay or runtime authority guarantees");
        }
        let before = self.state_hash.clone();
        self.rebuild_hash();
        let action = EditorActionRecord::new(
            self.history.undo_stack.len() as u64 + 1,
            action_type,
            &self.selection.ids,
            payload,
            &before,
        );
        self.history.push(action);
        self.rebuild_hash();
        Ok(())
    }

    fn rebuild_hash(&mut self) {
        let nodes = self
            .nodes
            .iter()
            .map(|node| format!("{}={}", node.id, node.content_hash))
            .collect::<Vec<_>>()
            .join(",");
        let clipboard = self
            .clipboard
            .iter()
            .map(|node| node.id.as_str())
            .collect::<Vec<_>>()
            .join(",");
        self.state_hash = stable_hash(&[
            "editing-engine",
            &nodes,
            &clipboard,
            &self.selection.selection_hash,
            &self.history.history_hash,
        ]);
    }
}
