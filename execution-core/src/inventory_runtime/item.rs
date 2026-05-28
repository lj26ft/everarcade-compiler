use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub item_id: String,
    pub item_root: String,
}
impl Item {
    pub fn new(id: &str) -> Self {
        Self {
            item_id: id.into(),
            item_root: format!("item:{id}:root"),
        }
    }
}
