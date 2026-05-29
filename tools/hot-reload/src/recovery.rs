use crate::stable_hash;

pub fn restore_after_reload(checkpoint_root: &str) -> String { stable_hash(&["hot-reload-restore", checkpoint_root]) }
