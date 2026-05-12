pub fn persistence_guarantee(checkpoint_count: usize, archival_copies: usize) -> bool {
    checkpoint_count > 0 && archival_copies > 0
}
