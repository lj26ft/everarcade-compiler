use super::{treaty::ExecutionTreaty, treaty_scope::domain_participates};

pub fn validate_treaty(treaty: &ExecutionTreaty) -> bool {
    !treaty.participating_domains.is_empty()
        && treaty
            .participating_domains
            .iter()
            .all(|d| domain_participates(treaty, *d))
}
