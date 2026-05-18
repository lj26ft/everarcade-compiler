use std::{fs, path::Path};

use anyhow::{anyhow, Result};

use crate::{
    canonical::load_manifest,
    federation::{
        bundle::{
            export_continuity_bundle, import_continuity_bundle, receipt_files,
            verify_continuity_bundle,
        },
        node::FederationNodeId,
    },
    lineage,
    operator::load_recovery_descriptor,
    persistence::{checkpoint_store, package_store, receipt_store},
    state::Hash256,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldRuntimeDescriptor {
    pub world_id: String,
    pub package_root: Hash256,
    pub current_checkpoint_root: Hash256,
    pub latest_execution_id: Hash256,
    pub latest_sequence: u64,
    pub frozen: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldMigrationRequest {
    pub source_node: FederationNodeId,
    pub destination_node: FederationNodeId,
    pub world_id: String,
    pub expected_package_root: Hash256,
    pub expected_checkpoint_root: Hash256,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldMigrationResult {
    pub migration_ok: bool,
    pub continuity_ok: bool,
    pub replay_ok: bool,
    pub resumed_ok: bool,
}

pub fn freeze_world(world_root: &Path, world_id: &str) -> Result<WorldRuntimeDescriptor> {
    fs::write(world_root.join("frozen.lock"), b"frozen")?;
    let descriptor = load_runtime_descriptor(world_root, world_id, true)?;
    Ok(descriptor)
}

pub fn resume_world(world_root: &Path, world_id: &str) -> Result<WorldRuntimeDescriptor> {
    let verification = verify_continuity_bundle(world_root)?;
    if !verification.bundle_ok {
        return Err(anyhow!("continuity verification failed"));
    }
    let lock = world_root.join("frozen.lock");
    if lock.exists() {
        let _ = fs::remove_file(lock);
    }
    load_runtime_descriptor(world_root, world_id, false)
}

pub fn validate_world_handoff(
    bundle_root: &Path,
    request: &WorldMigrationRequest,
) -> Result<WorldMigrationResult> {
    let v = verify_continuity_bundle(bundle_root)?;
    let manifest = load_manifest(&bundle_root.join("manifest.bin"))?;
    let lineage_chain = lineage::load_lineage(&bundle_root.join("lineage.bin"))?;
    let descriptor = load_recovery_descriptor(&bundle_root.join("descriptor.bin"))?;

    if manifest.package_root != request.expected_package_root {
        return Ok(WorldMigrationResult {
            migration_ok: false,
            continuity_ok: false,
            replay_ok: false,
            resumed_ok: false,
        });
    }
    if manifest.checkpoint_root != request.expected_checkpoint_root {
        return Ok(WorldMigrationResult {
            migration_ok: false,
            continuity_ok: false,
            replay_ok: false,
            resumed_ok: false,
        });
    }

    let sequence_continuous = lineage_chain
        .records
        .iter()
        .enumerate()
        .all(|(i, r)| r.sequence as usize == i + 1);
    let descriptor_ok = descriptor.package_root == manifest.package_root
        && descriptor.latest_checkpoint_root == manifest.checkpoint_root;

    Ok(WorldMigrationResult {
        migration_ok: v.bundle_ok && descriptor_ok && sequence_continuous,
        continuity_ok: v.bundle_ok && v.lineage_ok && descriptor_ok && sequence_continuous,
        replay_ok: v.receipts_ok,
        resumed_ok: false,
    })
}

pub fn migrate_world(
    source_world_root: &Path,
    destination_bundle_root: &Path,
    destination_world_root: &Path,
    request: &WorldMigrationRequest,
) -> Result<WorldMigrationResult> {
    let _ = freeze_world(source_world_root, &request.world_id)?;

    let receipts = receipt_files(&source_world_root.join("receipts"))?;
    export_continuity_bundle(
        destination_bundle_root,
        &source_world_root.join("package/world.wasm"),
        &source_world_root.join("checkpoint.bin"),
        &source_world_root.join("lineage.bin"),
        &receipts,
        &source_world_root.join("manifest.bin"),
        &source_world_root.join("descriptor.bin"),
    )?;

    import_continuity_bundle(destination_bundle_root, destination_world_root)?;
    let mut report = validate_world_handoff(destination_world_root, request)?;
    let resumed = resume_world(destination_world_root, &request.world_id)?;
    report.resumed_ok = !resumed.frozen;
    report.migration_ok = report.continuity_ok && report.replay_ok && report.resumed_ok;
    Ok(report)
}

fn load_runtime_descriptor(
    world_root: &Path,
    world_id: &str,
    frozen: bool,
) -> Result<WorldRuntimeDescriptor> {
    let package_bytes =
        package_store::load_package_bytes(&world_root.join("package/world.wasm"), None)?;
    let package_root = package_store::package_root(&package_bytes);
    let checkpoint = crate::state::decode_checkpoint(&checkpoint_store::load_checkpoint(
        &world_root.join("checkpoint.bin"),
        None,
    )?)?;
    let lineage_chain = lineage::load_lineage(&world_root.join("lineage.bin"))?;
    let latest_execution_id = lineage_chain
        .records
        .last()
        .map(|r| r.execution_id)
        .unwrap_or([0; 32]);
    let latest_sequence = lineage_chain
        .records
        .last()
        .map(|r| r.sequence)
        .unwrap_or(0);
    let _latest_receipt = receipt_store::load_receipt(
        &receipt_files(&world_root.join("receipts"))?
            .last()
            .ok_or_else(|| anyhow!("missing receipt"))?
            .to_path_buf(),
    )?;
    Ok(WorldRuntimeDescriptor {
        world_id: world_id.to_string(),
        package_root,
        current_checkpoint_root: checkpoint.root(),
        latest_execution_id,
        latest_sequence,
        frozen,
    })
}
