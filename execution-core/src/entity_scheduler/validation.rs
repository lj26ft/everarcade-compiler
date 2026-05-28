use super::runtime::EntitySchedulerRuntime;
pub fn validate_schedule(s: &EntitySchedulerRuntime) -> bool {
    s.replay_tip == format!("scheduler:replay:{}:{}", s.tick, s.last_order.join(","))
}
pub fn validate_schedule_equivalence(
    a: &EntitySchedulerRuntime,
    b: &EntitySchedulerRuntime,
) -> Result<(), &'static str> {
    if a == b && validate_schedule(a) {
        Ok(())
    } else {
        Err("scheduling divergence rejected")
    }
}
pub fn reject_scheduling_mutation(authorized: bool) -> Result<(), &'static str> {
    if authorized {
        Ok(())
    } else {
        Err("unauthorized scheduling mutation rejected")
    }
}
