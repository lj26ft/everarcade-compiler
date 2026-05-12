use serde::{Deserialize, Serialize};

pub type Hash = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArbitrationCase {
    pub case_id: Hash,
    pub participating_domains: Vec<Hash>,
    pub treaty_root: Hash,
    pub dispute_root: Hash,
    pub resolution_root: Hash,
}
