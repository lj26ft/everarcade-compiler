use everarcade_abi::State;

use crate::{
    codec::canonical_codec::canonical_encode, hash_runtime::canonical_hash::canonical_hash,
};

pub fn state_root(state: &State) -> String {
    canonical_hash(&canonical_encode(state))
}
