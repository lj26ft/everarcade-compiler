pub fn namespace_treaty_root(namespace_root: [u8; 32], treaty_root: [u8; 32]) -> [u8; 32] {
    let mut out = namespace_root;
    for (i, b) in treaty_root.iter().enumerate() {
        out[i] ^= *b;
    }
    out
}
