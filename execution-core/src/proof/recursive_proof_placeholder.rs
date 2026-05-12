use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecursiveProofPlaceholder {
    pub enabled: bool,
    pub note: String,
}
