use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntityProjection {
    pub entity_id: String,
    pub label: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeProjection {
    pub world_root: String,
    pub tick: u64,
    pub entities: Vec<EntityProjection>,
    pub simulation_activity: Vec<String>,
    pub replay_frame: Option<String>,
    pub projection_hash: String,
    pub authority_mutation: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewportState {
    pub selected_entity: Option<String>,
    pub camera_hash: String,
    pub last_projection_hash: String,
}

impl RuntimeProjection {
    pub fn sample() -> Self {
        project_runtime_state(
            "world-root",
            42,
            vec![
                EntityProjection {
                    entity_id: "entity:settler".into(),
                    label: "Settler".into(),
                    x: 4,
                    y: 7,
                },
                EntityProjection {
                    entity_id: "entity:workshop".into(),
                    label: "Workshop".into(),
                    x: 12,
                    y: 3,
                },
            ],
            vec![
                "ecs:movement".into(),
                "ai:civilization".into(),
                "partition:north".into(),
            ],
            Some("replay-frame-0042".into()),
        )
    }
}

pub fn project_runtime_state(
    world_root: &str,
    tick: u64,
    mut entities: Vec<EntityProjection>,
    mut simulation_activity: Vec<String>,
    replay_frame: Option<String>,
) -> RuntimeProjection {
    entities.sort_by(|a, b| a.entity_id.cmp(&b.entity_id));
    simulation_activity.sort();
    let mut parts = vec![
        "viewport-projection".to_owned(),
        world_root.to_owned(),
        tick.to_string(),
    ];
    parts.extend(entities.iter().map(|entity| {
        format!(
            "{}:{}:{}:{}",
            entity.entity_id, entity.label, entity.x, entity.y
        )
    }));
    parts.extend(
        simulation_activity
            .iter()
            .map(|activity| format!("activity:{activity}")),
    );
    if let Some(frame) = &replay_frame {
        parts.push(format!("replay:{frame}"));
    }
    RuntimeProjection {
        world_root: world_root.to_owned(),
        tick,
        entities,
        simulation_activity,
        replay_frame,
        projection_hash: stable_hash(&parts.iter().map(String::as_str).collect::<Vec<_>>()),
        authority_mutation: false,
    }
}

pub fn select_entity(
    state: &mut ViewportState,
    projection: &RuntimeProjection,
    entity_id: &str,
) -> Result<(), &'static str> {
    if projection
        .entities
        .iter()
        .any(|entity| entity.entity_id == entity_id)
    {
        state.selected_entity = Some(entity_id.to_owned());
        state.last_projection_hash = projection.projection_hash.clone();
        Ok(())
    } else {
        Err("selection must reference projected entity data")
    }
}

pub fn reject_authority_mutation(requested: bool) -> Result<(), &'static str> {
    if requested {
        Err("viewport consumes projection data only")
    } else {
        Ok(())
    }
}

pub fn renderer_is_authoritative() -> bool {
    false
}
