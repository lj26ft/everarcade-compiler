use serde::{Deserialize, Serialize};
use super::Hash;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationKeyRecord {
    pub verification_key_root: Hash,
}
