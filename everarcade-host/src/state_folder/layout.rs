use std::path::{Path, PathBuf};

pub fn required_paths(base: &Path) -> Vec<PathBuf> {
    ["packages","receipts","checkpoints","proofs","anchors","manifests","ipfs","xrpl"].iter().map(|s| base.join("state").join(s)).collect()
}
