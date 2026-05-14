use std::path::Path;

pub fn index_files_exist(state_root: &Path) -> bool {
    ["receipt.index", "checkpoint.index", "anchor.index"]
        .iter()
        .all(|n| state_root.join("manifests").join(n).exists())
}
