pub fn judicial_lineage(prior: [u8; 32], ruling: [u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for i in 0..32 { out[i] = prior[i].wrapping_add(ruling[i]); }
    out
}
