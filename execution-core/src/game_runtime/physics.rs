use super::{entities::Entity, input::PlayerInput};
pub fn apply_input(entity: &mut Entity, input: &PlayerInput) {
    entity.x += input.dx;
    entity.y += input.dy;
}
