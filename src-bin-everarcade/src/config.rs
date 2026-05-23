use std::{env, path::PathBuf};

pub fn runtime_root() -> PathBuf {
    env::current_dir().unwrap().join("runtime")
}
pub fn games_root() -> PathBuf {
    runtime_root().join("games")
}
