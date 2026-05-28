use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipRecord {
    pub item_id: String,
    pub owner_id: String,
    pub seq: u64,
    pub previous_owner: String,
    pub ownership_root: String,
}
