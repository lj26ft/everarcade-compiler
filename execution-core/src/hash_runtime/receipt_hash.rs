use everarcade_abi::ExecutionReceipt;

use crate::{
    codec::canonical_codec::canonical_encode, hash_runtime::canonical_hash::canonical_hash,
};

pub fn receipt_hash(receipt: &ExecutionReceipt) -> String {
    canonical_hash(&canonical_encode(receipt))
}
