use crate::types::Hash;

/// Deterministic hash (placeholder for real SHA-256 later)
pub fn hash_bytes(data: &[u8]) -> Hash {
    let mut state: u64 = 1469598103934665603;
    let prime: u64 = 1099511628211;

    for b in data {
        state ^= *b as u64;
        state = state.wrapping_mul(prime);
    }

    let mut out = [0u8; 32];
    let mut s = state;

    for i in 0..32 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        out[i] = (s >> ((i % 8) * 8)) as u8;
    }

    out
}

/// Combine multiple hashes deterministically
pub fn hash_combine(parts: &[&[u8]]) -> Hash {
    let mut combined = Vec::new();

    for p in parts {
        combined.extend_from_slice(p);
    }

    hash_bytes(&combined)
}

/// Convert hash → hex string
pub fn hash_to_hex(hash: &Hash) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut out = String::with_capacity(64);

    for &b in hash {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }

    out
}
