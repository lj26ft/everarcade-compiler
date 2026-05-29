use crate::stable_hash;

pub fn archive_replay_compatible_assets(package_hash: &str) -> String { stable_hash(&["archive", package_hash, "replay-compatible"] ) }
