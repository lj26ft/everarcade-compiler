use std::path::Path;

use super::index_scan::index_files_exist;

pub fn indexes_need_rebuild(state_root: &Path) -> bool {
    !index_files_exist(state_root)
}
