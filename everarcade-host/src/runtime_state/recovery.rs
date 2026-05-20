use std::{fs, path::Path};

use super::{error::RuntimeStateError, snapshot::RuntimeStateSnapshot};

pub fn load_latest_snapshot(
    world_root: &Path,
) -> Result<Option<RuntimeStateSnapshot>, RuntimeStateError> {
    let snapshots = world_root.join("state").join("snapshots");
    if !snapshots.exists() {
        return Ok(None);
    }
    let mut files: Vec<_> = fs::read_dir(snapshots)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    files.sort();
    let Some(last) = files.last() else {
        return Ok(None);
    };
    Ok(Some(serde_json::from_slice(&fs::read(last)?)?))
}

pub fn restore_snapshot(
    world_root: &Path,
    snapshot: &RuntimeStateSnapshot,
) -> Result<(), RuntimeStateError> {
    fs::create_dir_all(world_root.join("state").join("snapshots"))?;
    let path = world_root
        .join("state")
        .join("snapshots")
        .join(format!("{:020}.json", snapshot.checkpoint_sequence));
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(snapshot)?)?;
    fs::rename(tmp, path)?;
    Ok(())
}
