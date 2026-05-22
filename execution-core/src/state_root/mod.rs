use crate::hashing::sha256;
use std::collections::BTreeMap;

pub fn canonical_state_root(entries: &BTreeMap<String, Vec<u8>>) -> [u8; 32] {
    let canonical: Vec<(&String, &Vec<u8>)> = entries.iter().collect();
    let bytes = bincode::serialize(&canonical).expect("canonical state root serialization failed");
    sha256(&bytes)
}
