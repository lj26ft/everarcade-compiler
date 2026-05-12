use super::{fuel::FuelMeter, host_abi::HostFrame, memory_model::MemoryModel, transition::RuntimeTransition};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeReceipt {
    pub trace_hash: u64,
    pub final_memory_hash: u64,
    pub fuel_remaining: u64,
}

pub fn execute_canonical(input: &[u8], initial_fuel: u64) -> Option<RuntimeReceipt> {
    let frame = HostFrame::decode(input)?;
    let mut memory = MemoryModel::with_size(frame.payload.len().max(1));
    let _ = memory.write(0, &frame.payload);
    let mut fuel = FuelMeter::new(initial_fuel);
    if !fuel.charge(frame.payload.len() as u64) { return None; }
    let transition = RuntimeTransition::apply(0, frame.payload.len() as u64, memory.digest(), frame.payload.len() as u64);
    Some(RuntimeReceipt { trace_hash: transition.state_after, final_memory_hash: memory.digest(), fuel_remaining: fuel.remaining })
}
