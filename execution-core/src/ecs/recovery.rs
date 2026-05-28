use super::{
    runtime::{EcsError, EcsReplayWindow, EcsRuntime},
    ComponentValue, Entity,
};

pub fn restore_from_replay(window: &EcsReplayWindow) -> Result<EcsRuntime, EcsError> {
    let mut runtime = EcsRuntime::default();
    for event in &window.events {
        if !runtime
            .storage
            .entities
            .contains_key(&event.mutation.entity_id)
        {
            runtime.spawn(Entity::new(&event.mutation.entity_id));
        }
        runtime.set_component(
            &event.mutation.entity_id,
            ComponentValue::new(
                &event.mutation.component,
                event.resulting_value,
                &event.mutation.authority,
            ),
        );
        runtime.replay.events.push(event.clone());
        runtime.tick = runtime.tick.max(event.tick + 1);
    }
    Ok(runtime)
}
