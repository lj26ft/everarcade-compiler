use super::ownership::OwnershipRecord;
pub fn transfer_ownership(prev: &OwnershipRecord, new_owner: &str) -> OwnershipRecord {
    let seq = prev.seq + 1;
    OwnershipRecord {
        item_id: prev.item_id.clone(),
        owner_id: new_owner.into(),
        seq,
        previous_owner: prev.owner_id.clone(),
        ownership_root: format!(
            "ownership:{}:{seq}:{}:{}",
            prev.item_id, prev.owner_id, new_owner
        ),
    }
}
