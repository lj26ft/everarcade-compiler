pub fn validate_wasm_execution_environment(memory_limit: usize, fuel_limit: u64) -> bool {
    memory_limit > 0 && fuel_limit > 0
}
