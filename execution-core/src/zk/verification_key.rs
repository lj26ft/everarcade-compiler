use super::Hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationKeyRecord {
    pub verification_key_root: Hash,
}
