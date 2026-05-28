use super::{
    ownership::OwnershipRecord, transfer::transfer_ownership, validation::validate_inventory,
};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryRuntime {
    pub records: Vec<OwnershipRecord>,
    pub continuity_root: String,
}
impl InventoryRuntime {
    pub fn genesis(item: &str, owner: &str) -> Self {
        let r = OwnershipRecord {
            item_id: item.into(),
            owner_id: owner.into(),
            seq: 0,
            previous_owner: "genesis".into(),
            ownership_root: format!("ownership:{item}:0:{owner}"),
        };
        Self {
            continuity_root: format!("inventory:continuity:{}", r.ownership_root),
            records: vec![r],
        }
    }
    pub fn transfer(&mut self, new_owner: &str) -> Result<(), &'static str> {
        let next = transfer_ownership(self.records.last().expect("inventory genesis"), new_owner);
        self.continuity_root = format!("inventory:continuity:{}", next.ownership_root);
        self.records.push(next);
        if validate_inventory(self) {
            Ok(())
        } else {
            Err("ownership divergence rejected")
        }
    }
}
