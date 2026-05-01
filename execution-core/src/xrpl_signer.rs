use crate::hash::hash_bytes;

fn to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut out = String::with_capacity(bytes.len() * 2);

    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }

    out
}

pub fn sign(payload: &[u8]) -> String {
    let hash = hash_bytes(payload);
    let hex = to_hex(&hash);

    let mut out = String::from("SIG_");
    out.push_str(&hex);

    out
}

pub fn verify(payload: &[u8], sig: &str) -> bool {
    sign(payload) == sig
}
