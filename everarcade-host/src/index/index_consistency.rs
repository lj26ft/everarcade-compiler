use std::{fs, path::Path};

pub fn latest_roots_from_truth(state_root: &Path) -> (Option<String>, Option<String>) {
    let mut r: Vec<String> = fs::read_dir(state_root.join("receipts"))
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.path()
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .collect();
    let mut c: Vec<String> = fs::read_dir(state_root.join("checkpoints"))
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.path()
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .collect();
    r.sort();
    c.sort();
    (r.pop(), c.pop())
}
