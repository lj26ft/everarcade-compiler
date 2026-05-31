use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCursor {
    pub next_sequence: u64,
    pub continuity_root: String,
}
