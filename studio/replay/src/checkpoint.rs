pub fn inspect_checkpoint(root: &str) -> String { crate::stable_hash(&["checkpoint", root, "read-only"] ) }
