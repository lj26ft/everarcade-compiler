use crate::{stable_hash, viewport::RuntimeProjection};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CameraCommand {
    Orbit {
        yaw_degrees: i32,
        pitch_degrees: i32,
    },
    Pan {
        x: i32,
        y: i32,
    },
    Zoom {
        level_percent: i32,
    },
    FocusSelected,
}

impl CameraCommand {
    fn as_hash_part(&self) -> String {
        match self {
            Self::Orbit {
                yaw_degrees,
                pitch_degrees,
            } => {
                format!("orbit:{yaw_degrees}:{pitch_degrees}")
            }
            Self::Pan { x, y } => format!("pan:{x}:{y}"),
            Self::Zoom { level_percent } => format!("zoom:{level_percent}"),
            Self::FocusSelected => "focus-selected".to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PickResult {
    pub entity_id: String,
    pub hovered: bool,
    pub selected: bool,
    pub highlight_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarqueeRect {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl MarqueeRect {
    pub fn normalized(mut self) -> Self {
        if self.min_x > self.max_x {
            std::mem::swap(&mut self.min_x, &mut self.max_x);
        }
        if self.min_y > self.max_y {
            std::mem::swap(&mut self.min_y, &mut self.max_y);
        }
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InteractiveViewportState {
    pub selected_entities: Vec<String>,
    pub hovered_entity: Option<String>,
    pub camera_commands: Vec<CameraCommand>,
    pub projection_hash: String,
    pub replay_safe: bool,
    pub projection_only: bool,
    pub authority_mutation: bool,
}

impl InteractiveViewportState {
    pub fn from_projection(projection: &RuntimeProjection) -> Self {
        Self {
            selected_entities: Vec::new(),
            hovered_entity: None,
            camera_commands: Vec::new(),
            projection_hash: projection.projection_hash.clone(),
            replay_safe: true,
            projection_only: true,
            authority_mutation: false,
        }
    }

    pub fn pick(
        &mut self,
        projection: &RuntimeProjection,
        x: i32,
        y: i32,
    ) -> Result<PickResult, &'static str> {
        let entity = projection
            .entities
            .iter()
            .filter(|entity| (entity.x - x).abs() <= 1 && (entity.y - y).abs() <= 1)
            .min_by(|left, right| left.entity_id.cmp(&right.entity_id))
            .ok_or("mouse picking must hit projected entity data")?;
        self.hovered_entity = Some(entity.entity_id.clone());
        self.selected_entities = vec![entity.entity_id.clone()];
        self.projection_hash = projection.projection_hash.clone();
        Ok(PickResult {
            entity_id: entity.entity_id.clone(),
            hovered: true,
            selected: true,
            highlight_hash: stable_hash(&[
                "highlight",
                &entity.entity_id,
                &projection.projection_hash,
            ]),
        })
    }

    pub fn marquee_select(
        &mut self,
        projection: &RuntimeProjection,
        rect: MarqueeRect,
    ) -> Vec<String> {
        let rect = rect.normalized();
        let mut selected: Vec<String> = projection
            .entities
            .iter()
            .filter(|entity| {
                entity.x >= rect.min_x
                    && entity.x <= rect.max_x
                    && entity.y >= rect.min_y
                    && entity.y <= rect.max_y
            })
            .map(|entity| entity.entity_id.clone())
            .collect();
        selected.sort();
        self.selected_entities = selected.clone();
        self.projection_hash = projection.projection_hash.clone();
        selected
    }

    pub fn apply_camera_command(&mut self, command: CameraCommand) -> String {
        self.camera_commands.push(command);
        let mut parts = vec![
            "interactive-camera".to_owned(),
            self.projection_hash.clone(),
        ];
        parts.extend(self.camera_commands.iter().map(CameraCommand::as_hash_part));
        stable_hash(&parts.iter().map(String::as_str).collect::<Vec<_>>())
    }

    pub fn hover_feedback(&self) -> Option<String> {
        self.hovered_entity
            .as_ref()
            .map(|entity| stable_hash(&["hover", entity, &self.projection_hash]))
    }
}

pub fn viewport_selection_equivalence() -> bool {
    let projection = RuntimeProjection::sample();
    let mut first = InteractiveViewportState::from_projection(&projection);
    let mut second = InteractiveViewportState::from_projection(&projection);
    first.pick(&projection, 4, 7).ok() == second.pick(&projection, 4, 7).ok()
        && first.marquee_select(
            &projection,
            MarqueeRect {
                min_x: 0,
                min_y: 0,
                max_x: 20,
                max_y: 20,
            },
        ) == second.marquee_select(
            &projection,
            MarqueeRect {
                min_x: 20,
                min_y: 20,
                max_x: 0,
                max_y: 0,
            },
        )
        && first.projection_only
        && first.replay_safe
        && !first.authority_mutation
}

pub fn camera_controls_equivalence() -> bool {
    let projection = RuntimeProjection::sample();
    let mut first = InteractiveViewportState::from_projection(&projection);
    let mut second = InteractiveViewportState::from_projection(&projection);
    for command in [
        CameraCommand::Orbit {
            yaw_degrees: 30,
            pitch_degrees: -15,
        },
        CameraCommand::Pan { x: 8, y: -4 },
        CameraCommand::Zoom { level_percent: 125 },
        CameraCommand::FocusSelected,
    ] {
        first.apply_camera_command(command.clone());
        second.apply_camera_command(command);
    }
    first == second
}

pub fn reject_authority_mutation(requested: bool) -> Result<(), &'static str> {
    if requested {
        Err("interactive viewport is projection-only and cannot mutate runtime authority")
    } else {
        Ok(())
    }
}
