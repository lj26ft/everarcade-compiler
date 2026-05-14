use std::{fs, path::Path};

pub fn latest_root_from_dir(state_root: &Path, folder: &str) -> Option<String> {
    let dir = state_root.join(folder);
    let mut ids: Vec<String> = fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.path()
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .collect();
    ids.sort();
    ids.pop()
}
