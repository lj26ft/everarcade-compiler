#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldManifest { pub world_id: String, pub manifest_hash: String, pub replay_compatible: bool }
pub fn create_world(world_id: &str, terrain_hash: &str, placements_hash: &str, runtime_params: &[&str]) -> WorldManifest {
    let mut parts = vec!["world-manifest", world_id, terrain_hash, placements_hash]; parts.extend_from_slice(runtime_params);
    WorldManifest { world_id: world_id.to_owned(), manifest_hash: crate::stable_hash(&parts), replay_compatible: true }
}
