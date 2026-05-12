use super::deterministic_runtime::{execute_canonical, RuntimeReceipt};

pub fn replay_for_verifier(input: &[u8], fuel: u64) -> Option<RuntimeReceipt> {
    execute_canonical(input, fuel)
}
