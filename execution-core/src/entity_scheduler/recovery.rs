use super::{runtime::EntitySchedulerRuntime, validation::validate_schedule_equivalence};
pub fn restore_schedule(
    a: &EntitySchedulerRuntime,
    b: &EntitySchedulerRuntime,
) -> Result<EntitySchedulerRuntime, &'static str> {
    validate_schedule_equivalence(a, b)?;
    Ok(b.clone())
}
