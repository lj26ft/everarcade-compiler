use sha2::{Digest, Sha256};
use std::{fs, path::Path};

pub use everarcade_economy_sdk::EconomyHooks;
pub use everarcade_entity_sdk::EntityHooks;
pub use everarcade_governance_sdk::GovernanceHooks;
pub use everarcade_simulation_sdk::SimulationHooks;
pub use everarcade_world_sdk::WorldHooks;

pub fn load_game_manifest(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
pub fn verify_manifest_determinism(body: &str) -> bool {
    body.contains("deterministic")
}
pub fn build_game_package(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
pub fn verify_package_determinism(a: &[u8], b: &[u8]) -> bool {
    build_game_package(a) == build_game_package(b)
}
pub fn export_runtime_bundle(path: &Path, body: &[u8]) -> Result<(), std::io::Error> {
    fs::write(path, body)
}
pub fn register_asset(
    manifest_dir: &Path,
    asset_name: &str,
    asset_bytes: &[u8],
) -> Result<String, std::io::Error> {
    fs::create_dir_all(manifest_dir)?;
    let hash = hex::encode(Sha256::digest(asset_bytes));
    fs::write(manifest_dir.join(format!("{asset_name}.hash")), &hash)?;
    Ok(hash)
}
pub fn verify_asset_integrity(expected_hash: &str, asset_bytes: &[u8]) -> bool {
    expected_hash == hex::encode(Sha256::digest(asset_bytes))
}
pub fn replay_asset_manifest(manifest_dir: &Path) -> Result<Vec<String>, std::io::Error> {
    let mut entries = vec![];
    for e in fs::read_dir(manifest_dir)? {
        entries.push(e?.file_name().to_string_lossy().to_string());
    }
    entries.sort();
    Ok(entries)
}

pub fn start_local_federation(root: &Path) -> Result<(), std::io::Error> {
    for n in ["node-a", "node-b", "node-c"] {
        fs::create_dir_all(root.join(n))?;
    }
    Ok(())
}
pub fn replay_local_world(root: &Path) -> Result<(), std::io::Error> {
    fs::write(root.join("local-replay.log"), "ok")
}
pub fn inspect_local_continuity(root: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(root.join("local-replay.log"))
}
pub fn inspect_replay_timeline(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
pub fn inspect_entity_evolution(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
pub fn inspect_interaction_trace(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub struct DeveloperRuntimeHooks<T> {
    pub world: T,
}
impl<T> DeveloperRuntimeHooks<T>
where
    T: WorldHooks + EntityHooks + SimulationHooks + EconomyHooks + GovernanceHooks,
{
    pub fn on_world_tick(&mut self, tick: u64) {
        self.world.on_world_tick(tick)
    }
    pub fn on_entity_spawn(&mut self, entity_id: &str) {
        self.world.on_entity_spawn(entity_id)
    }
    pub fn on_interaction(&mut self, source: &str, target: &str, action: &str) {
        self.world.on_interaction(source, target, action)
    }
    pub fn on_partition_migrate(&mut self, entity_id: &str, partition: &str) {
        self.world.on_partition_migrate(entity_id, partition)
    }
    pub fn on_settlement(&mut self, tx_id: &str, amount_drops: u64) {
        self.world.on_settlement(tx_id, amount_drops)
    }
    pub fn on_governance_event(&mut self, event_id: &str) {
        self.world.on_governance_event(event_id)
    }
}
