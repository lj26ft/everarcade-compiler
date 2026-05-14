use std::path::{Path, PathBuf};

pub fn required_paths(base: &Path) -> Vec<PathBuf> {
    [
        "packages",
        "receipts",
        "checkpoints",
        "anchors",
        "manifests",
        "proofs",
        "ipfs",
        "xrpl",
    ]
    .iter()
    .map(|s| base.join(s))
    .collect()
}
