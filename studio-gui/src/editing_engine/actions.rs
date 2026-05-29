use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorActionType {
    Copy,
    Paste,
    Duplicate,
    Delete,
    Context(String),
    ComponentMutation,
    TerrainMutation,
    WorldMetadataMutation,
    DragDropPlacement,
    TransformMutation,
    SaveLoad,
    PlayMode,
    Publish,
}

impl EditorActionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Copy => "copy",
            Self::Paste => "paste",
            Self::Duplicate => "duplicate",
            Self::Delete => "delete",
            Self::Context(action) => action.as_str(),
            Self::ComponentMutation => "component-mutation",
            Self::TerrainMutation => "terrain-mutation",
            Self::WorldMetadataMutation => "world-metadata-mutation",
            Self::DragDropPlacement => "drag-drop-placement",
            Self::TransformMutation => "transform-mutation",
            Self::SaveLoad => "save-load",
            Self::PlayMode => "play-mode",
            Self::Publish => "publish",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorActionRecord {
    pub sequence: u64,
    pub action_type: EditorActionType,
    pub subjects: Vec<String>,
    pub payload: String,
    pub before_hash: String,
    pub after_hash: String,
    pub replay_lineage: String,
    pub undoable: bool,
    pub deterministic: bool,
}

impl EditorActionRecord {
    pub fn new(
        sequence: u64,
        action_type: EditorActionType,
        subjects: &[String],
        payload: &str,
        before_hash: &str,
    ) -> Self {
        let mut subjects = subjects.to_vec();
        subjects.sort();
        let subject_part = subjects.join(",");
        let after_hash = stable_hash(&[
            "editor-action",
            &sequence.to_string(),
            action_type.as_str(),
            &subject_part,
            payload,
            before_hash,
        ]);
        let replay_lineage = stable_hash(&["editor-replay-lineage", before_hash, &after_hash]);
        Self {
            sequence,
            action_type,
            subjects,
            payload: payload.to_owned(),
            before_hash: before_hash.to_owned(),
            after_hash,
            replay_lineage,
            undoable: true,
            deterministic: true,
        }
    }
}
