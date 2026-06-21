//! Standalone Tier-2 proof kernel.
//!
//! This crate is intentionally self-contained: no EverArcade monorepo imports,
//! no network calls, and no hidden fixtures. Harness tests load packaged fixture
//! text and recompute commitments with this deterministic kernel.

/// FNV-1a based deterministic commitment rendered as 32 bytes of lowercase hex.
///
/// The proof pack uses this compact hash to make replay, restore, migration,
/// federation, and JS/Rust equivalence checks independently reproducible without
/// linking to the EverArcade monorepo or external dependencies.
pub fn commitment(parts: &[&str]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }

    let mut out = String::with_capacity(64);
    for lane in 0..4u64 {
        let lane_hash = hash
            .rotate_left((lane * 13) as u32)
            .wrapping_add(0x9e37_79b9_7f4a_7c15u64.wrapping_mul(lane + 1));
        out.push_str(&format!("{lane_hash:016x}"));
    }
    out
}

pub fn world_hash(state_root: &str, receipt_root: &str, continuity_root: &str) -> String {
    commitment(&["world", state_root, receipt_root, continuity_root])
}

pub fn continuity_root(previous: &str, operation_root: &str) -> String {
    commitment(&["continuity", previous, operation_root])
}

pub fn fixture_value(text: &str, key: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (left, right) = line.split_once('=')?;
        (left.trim() == key).then(|| right.trim().to_string())
    })
}

pub fn require_value(text: &str, key: &str) -> String {
    fixture_value(text, key).unwrap_or_else(|| panic!("missing fixture key {key}"))
}
