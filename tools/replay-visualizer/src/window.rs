#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayWindow { pub start_tick: u64, pub end_tick: u64 }

pub fn inspect_window(start_tick: u64, end_tick: u64) -> Result<ReplayWindow, &'static str> {
    if start_tick <= end_tick { Ok(ReplayWindow { start_tick, end_tick }) } else { Err("replay window cannot run backwards") }
}
