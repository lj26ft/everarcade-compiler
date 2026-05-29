pub fn terrain_manifest(seed: &str, tiles: &[&str]) -> String { let mut parts = vec!["terrain", seed]; parts.extend_from_slice(tiles); crate::stable_hash(&parts) }
