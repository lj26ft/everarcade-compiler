use super::{archive_package::CivilizationArchivePackage, archive_root::archive_root};
pub fn validate_archive(pkg: &CivilizationArchivePackage) -> bool {
    pkg.archive_root == archive_root(pkg.civilization_root, pkg.replay_summary_root, pkg.checkpoint_root, pkg.continuity_root)
}
