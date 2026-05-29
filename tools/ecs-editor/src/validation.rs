use crate::{diagnostic, ecs_editor, CreatorDiagnostic};

pub fn validate_ecs_authoring() -> CreatorDiagnostic { diagnostic("ecs-authoring-validation", &["canonical-entity", "canonical-component", "ordered-system"] ) }

pub fn ecs_authoring_equivalence(components: &[&str]) -> bool {
    ecs_editor::archetype::visualize_archetype(components) == ecs_editor::archetype::visualize_archetype(components)
}

pub fn reject_hidden_ecs_mutation(hidden: bool) -> Result<(), &'static str> {
    if hidden { Err("hidden ECS mutation is not replay safe") } else { Ok(()) }
}
