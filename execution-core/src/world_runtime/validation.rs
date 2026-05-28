use super::world::PersistentWorld;

pub fn validate_world_continuity(world: &PersistentWorld) -> bool {
    world.continuity_root
        == format!(
            "world:{}:continuity:{}:{}:{}",
            world.world_id, world.tick, world.state_root, world.replay_tip
        )
}

pub fn reject_authority_mutation(attempted_authority: bool) -> Result<(), &'static str> {
    if attempted_authority {
        Err("unauthorized world mutation rejected")
    } else {
        Ok(())
    }
}

pub fn validate_world_equivalence(
    a: &PersistentWorld,
    b: &PersistentWorld,
) -> Result<(), &'static str> {
    if a == b && validate_world_continuity(a) {
        Ok(())
    } else {
        Err("divergent world restoration rejected")
    }
}
