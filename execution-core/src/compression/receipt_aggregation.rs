use sha2::{Digest, Sha256};

use super::replay_summary::Hash;

pub fn aggregate_receipts(mut receipts: Vec<Hash>) -> Hash {
    receipts.sort();
    let mut hasher = Sha256::new();
    for receipt in receipts {
        hasher.update(receipt);
    }
    hasher.finalize().into()
}
