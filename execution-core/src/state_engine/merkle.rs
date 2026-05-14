use sha2::{Digest, Sha256};

pub fn hash_leaf(key: &str, value: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update((key.len() as u64).to_le_bytes());
    hasher.update(key.as_bytes());
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value.as_bytes());
    hasher.finalize().into()
}

pub fn hash_branch(left_child_hash: &[u8; 32], right_child_hash: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(left_child_hash);
    hasher.update(right_child_hash);
    hasher.finalize().into()
}

pub fn compute_state_root(entries: &[(String, String)]) -> [u8; 32] {
    if entries.is_empty() {
        return Sha256::digest([]).into();
    }

    let mut level: Vec<[u8; 32]> = entries.iter().map(|(k, v)| hash_leaf(k, v)).collect();

    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        for pair in level.chunks(2) {
            let right = if pair.len() == 2 { pair[1] } else { pair[0] };
            next.push(hash_branch(&pair[0], &right));
        }
        level = next;
    }

    level[0]
}

pub fn to_hex(hash: &[u8; 32]) -> String {
    hex::encode(hash)
}
