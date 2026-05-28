use crate::gameplay::{GameplayRuntime, GameplayRuntimeError};
pub fn validate_recovered_runtime(runtime: &GameplayRuntime) -> Result<(), GameplayRuntimeError> {
    crate::gameplay::validation::validate_runtime(runtime)
}
