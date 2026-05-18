use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

use crate::{
    canonical::load_manifest,
    continuity::{restore_lineage_chain, ChainRestoreInput},
    lineage,
    operator::{load_recovery_descriptor, recover_world, OperatorRecoveryInput},
    persistence::{checkpoint_store, package_store, receipt_store},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityBundleLayout {
    pub root: PathBuf,
    pub manifest_path: PathBuf,
    pub lineage_path: PathBuf,
    pub checkpoint_path: PathBuf,
    pub descriptor_path: PathBuf,
    pub package_path: PathBuf,
    pub receipts_dir: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityBundleVerification {
    pub bundle_ok: bool,
    pub manifest_ok: bool,
    pub lineage_ok: bool,
    pub checkpoint_ok: bool,
    pub package_ok: bool,
    pub receipts_ok: bool,
    pub recovery_ok: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuityBundleMismatch {
    pub field: String,
    pub expected: String,
    pub actual: String,
}

pub fn export_continuity_bundle(
    destination: &Path,
    package_path: &Path,
    checkpoint_path: &Path,
    lineage_path: &Path,
    receipt_paths: &[PathBuf],
    manifest_path: &Path,
    descriptor_path: &Path,
) -> Result<ContinuityBundleVerification> {
    let layout = bundle_layout(destination);
    fs::create_dir_all(
        &layout
            .package_path
            .parent()
            .ok_or_else(|| anyhow!("invalid package path"))?,
    )?;
    fs::create_dir_all(&layout.receipts_dir)?;
    copy_and_verify(package_path, &layout.package_path)?;
    copy_and_verify(checkpoint_path, &layout.checkpoint_path)?;
    copy_and_verify(lineage_path, &layout.lineage_path)?;
    copy_and_verify(manifest_path, &layout.manifest_path)?;
    copy_and_verify(descriptor_path, &layout.descriptor_path)?;

    for (i, receipt_path) in sorted_receipts(receipt_paths).iter().enumerate() {
        let dest = layout.receipts_dir.join(format!("receipt_{:016}.bin", i));
        copy_and_verify(receipt_path, &dest)?;
    }

    verify_continuity_bundle(destination)
}

pub fn import_continuity_bundle(
    bundle_root: &Path,
    destination_world_root: &Path,
) -> Result<ContinuityBundleVerification> {
    let verification = verify_continuity_bundle(bundle_root)?;
    if !verification.bundle_ok {
        return Err(anyhow!("bundle verification failed before import"));
    }
    let src = bundle_layout(bundle_root);
    let dst = bundle_layout(destination_world_root);
    fs::create_dir_all(
        dst.package_path
            .parent()
            .ok_or_else(|| anyhow!("invalid package path"))?,
    )?;
    fs::create_dir_all(&dst.receipts_dir)?;

    copy_and_verify(&src.package_path, &dst.package_path)?;
    copy_and_verify(&src.checkpoint_path, &dst.checkpoint_path)?;
    copy_and_verify(&src.lineage_path, &dst.lineage_path)?;
    copy_and_verify(&src.manifest_path, &dst.manifest_path)?;
    copy_and_verify(&src.descriptor_path, &dst.descriptor_path)?;

    for receipt_path in receipt_files(&src.receipts_dir)? {
        let name = receipt_path
            .file_name()
            .ok_or_else(|| anyhow!("receipt file without name"))?;
        copy_and_verify(&receipt_path, &dst.receipts_dir.join(name))?;
    }

    verify_continuity_bundle(destination_world_root)
}

pub fn verify_continuity_bundle(bundle_root: &Path) -> Result<ContinuityBundleVerification> {
    let layout = bundle_layout(bundle_root);
    let receipt_paths = receipt_files(&layout.receipts_dir)?;
    ensure_receipt_names_deterministic(&receipt_paths)?;

    let package_bytes = package_store::load_package_bytes(&layout.package_path, None)?;
    let package_root = package_store::package_root(&package_bytes);
    let checkpoint_bytes = checkpoint_store::load_checkpoint(&layout.checkpoint_path, None)?;
    let checkpoint = crate::state::decode_checkpoint(&checkpoint_bytes)?;
    let lineage_chain = lineage::load_lineage(&layout.lineage_path)?;
    let lineage_report = lineage::validate_lineage_chain(&lineage_chain)?;
    let manifest = load_manifest(&layout.manifest_path)?;
    let descriptor = load_recovery_descriptor(&layout.descriptor_path)?;

    let restore = restore_lineage_chain(ChainRestoreInput {
        package_path: layout.package_path.clone(),
        checkpoint_path: layout.checkpoint_path.clone(),
        lineage_path: layout.lineage_path.clone(),
        receipt_paths: receipt_paths.clone(),
    })?;

    let descriptor_parent = layout
        .descriptor_path
        .parent()
        .ok_or_else(|| anyhow!("descriptor has no parent"))?;
    let recovery = recover_world(OperatorRecoveryInput {
        package_path: layout.package_path.clone(),
        checkpoint_path: layout.checkpoint_path.clone(),
        lineage_path: layout.lineage_path.clone(),
        receipt_paths: receipt_paths.clone(),
        descriptor_output_path: descriptor_parent.join("verify_recovery_descriptor.bin"),
    })?;

    let manifest_ok = manifest.package_root == package_root
        && manifest.checkpoint_root == checkpoint.root()
        && manifest.lineage_hash == crate::canonical::hashes::lineage_hash(&lineage_chain)
        && manifest.final_state_root == restore.final_state_root;
    let descriptor_ok = descriptor.package_root == package_root
        && descriptor.latest_checkpoint_root == checkpoint.root();

    let bundle_ok = manifest_ok
        && lineage_report.lineage_ok
        && restore.checkpoint_match
        && restore.lineage_match
        && restore.receipts_match
        && descriptor_ok
        && recovery.report.recovery_ok;

    Ok(ContinuityBundleVerification {
        bundle_ok,
        manifest_ok,
        lineage_ok: lineage_report.lineage_ok,
        checkpoint_ok: restore.checkpoint_match,
        package_ok: lineage_chain.package_root == package_root,
        receipts_ok: restore.receipts_match,
        recovery_ok: descriptor_ok && recovery.report.recovery_ok,
    })
}

fn bundle_layout(root: &Path) -> ContinuityBundleLayout {
    ContinuityBundleLayout {
        root: root.to_path_buf(),
        manifest_path: root.join("manifest.bin"),
        lineage_path: root.join("lineage.bin"),
        checkpoint_path: root.join("checkpoint.bin"),
        descriptor_path: root.join("descriptor.bin"),
        package_path: root.join("package").join("world.wasm"),
        receipts_dir: root.join("receipts"),
    }
}

fn sorted_receipts(receipts: &[PathBuf]) -> Vec<PathBuf> {
    let mut sorted = receipts.to_vec();
    sorted.sort();
    sorted
}

fn receipt_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.is_file())
        .collect();
    files.sort();
    Ok(files)
}

fn ensure_receipt_names_deterministic(receipts: &[PathBuf]) -> Result<()> {
    for (i, receipt_path) in receipts.iter().enumerate() {
        let expected = format!("receipt_{:016}.bin", i);
        let actual = receipt_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("invalid receipt file name"))?;
        if actual != expected {
            return Err(anyhow!(
                "receipt order/name mismatch: expected {expected} got {actual}"
            ));
        }
        let _ = receipt_store::load_receipt(receipt_path)?;
    }
    Ok(())
}

fn copy_and_verify(source: &Path, destination: &Path) -> Result<()> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    let src = fs::read(source)?;
    fs::write(destination, &src)?;
    let dst = fs::read(destination)?;
    if src != dst {
        return Err(anyhow!(
            "copied bytes mismatch for {}",
            destination.display()
        ));
    }
    Ok(())
}
