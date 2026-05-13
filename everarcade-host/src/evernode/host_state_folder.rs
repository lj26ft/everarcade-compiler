use std::path::{Path,PathBuf}; pub fn state_root(base:&Path)->PathBuf{base.join("state")}
