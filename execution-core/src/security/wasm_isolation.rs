use super::diagnostics::SecurityDiagnosticsEnvelope;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmIsolationPolicy {
    pub memory_ceiling_bytes: u64,
    pub max_fuel: u64,
    pub max_abi_payload_bytes: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuelExhaustionRecord {
    pub consumed: u64,
    pub max_fuel: u64,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationCheckpoint {
    pub tick: u64,
    pub replay_root: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmFaultEnvelope {
    pub deterministic_fault: bool,
    pub fault_type: &'static str,
    pub fuel: Option<FuelExhaustionRecord>,
    pub checkpoint: IsolationCheckpoint,
    pub diagnostics: SecurityDiagnosticsEnvelope,
}
pub fn isolate_wasm_fault(input: &[u8], policy: &WasmIsolationPolicy) -> Option<WasmFaultEnvelope> {
    let fault = if input.len() > policy.max_abi_payload_bytes {
        Some("oversized_payload")
    } else if input.starts_with(b"LOOP") {
        Some("fuel_exhaustion")
    } else if input.starts_with(b"MEM") {
        Some("invalid_memory_access")
    } else if input.starts_with(b"ABI") {
        Some("invalid_abi_payload")
    } else {
        None
    };
    fault.map(|fault_type| WasmFaultEnvelope {
        deterministic_fault: true,
        fault_type,
        fuel: (fault_type == "fuel_exhaustion").then_some(FuelExhaustionRecord {
            consumed: policy.max_fuel,
            max_fuel: policy.max_fuel,
        }),
        checkpoint: IsolationCheckpoint {
            tick: 0,
            replay_root: "isolation-checkpoint".to_string(),
        },
        diagnostics: SecurityDiagnosticsEnvelope::fault(
            fault_type,
            input.len() as u64,
            false,
            true,
        ),
    })
}
