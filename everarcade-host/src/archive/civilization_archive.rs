use super::archive_package::{CivilizationArchivePackage, Hash};
use super::archive_root::archive_root;
pub fn package(
    civilization_root: Hash,
    replay_summary_root: Hash,
    checkpoint_root: Hash,
    continuity_root: Hash,
) -> CivilizationArchivePackage {
    CivilizationArchivePackage {
        archive_root: archive_root(
            civilization_root,
            replay_summary_root,
            checkpoint_root,
            continuity_root,
        ),
        civilization_root,
        replay_summary_root,
        checkpoint_root,
        continuity_root,
    }
}
