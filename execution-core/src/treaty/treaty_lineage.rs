use super::treaty::Hash;

pub fn lineage_preserved(parent_treaty: Hash, child_parent_treaty: Option<Hash>) -> bool { child_parent_treaty == Some(parent_treaty) }
