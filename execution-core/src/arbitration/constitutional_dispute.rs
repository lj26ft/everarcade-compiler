pub fn dispute_is_constitutional(dispute_root: [u8; 32]) -> bool {
    dispute_root != [0; 32]
}
