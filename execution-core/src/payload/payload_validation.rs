use super::execution_payload::{ExecutionPayload, Mutation};

pub fn validate_payload(payload: &ExecutionPayload) -> Result<(), String> {
    let mutations = payload.mutations();
    if mutations.is_empty() {
        return Err("payload must contain at least one mutation".to_string());
    }

    let mut prior: Option<&Mutation> = None;
    for mutation in mutations {
        if mutation.key.is_empty() {
            return Err("mutation key must not be empty".to_string());
        }

        if let Some(prev) = prior {
            if prev.key == mutation.key {
                return Err(format!("duplicate mutation key detected: {}", mutation.key));
            }
        }
        prior = Some(mutation);
    }

    Ok(())
}
