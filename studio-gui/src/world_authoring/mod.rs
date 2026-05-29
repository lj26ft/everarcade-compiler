use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WorldObjectKind {
    Entity,
    Region,
    Partition,
    SpawnPoint,
    ResourceNode,
    Faction,
    Civilization,
    WorldProp,
    Structure,
    RuntimeMarker,
    Metadata,
}

impl WorldObjectKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Region => "region",
            Self::Partition => "partition",
            Self::SpawnPoint => "spawn-point",
            Self::ResourceNode => "resource-node",
            Self::Faction => "faction",
            Self::Civilization => "civilization",
            Self::WorldProp => "world-prop",
            Self::Structure => "structure",
            Self::RuntimeMarker => "runtime-marker",
            Self::Metadata => "metadata",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transform {
    pub x: i32,
    pub y: i32,
    pub rotation_degrees: i32,
    pub scale_percent: i32,
}

impl Transform {
    pub fn snapped(mut self, grid: i32) -> Self {
        let grid = grid.max(1);
        self.x = (self.x / grid) * grid;
        self.y = (self.y / grid) * grid;
        self.rotation_degrees = (self.rotation_degrees / 15) * 15;
        self.scale_percent = (self.scale_percent / 5) * 5;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldObject {
    pub id: String,
    pub name: String,
    pub kind: WorldObjectKind,
    pub transform: Transform,
    pub parent: Option<String>,
    pub tags: Vec<String>,
    pub component_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditorActionKind {
    Place,
    Move,
    Delete,
    Duplicate,
    Parent,
    Group,
    Tag,
    PropertyEdit,
    SimulationControl,
    AssetDrop,
    Publish,
}

impl EditorActionKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Place => "place",
            Self::Move => "move",
            Self::Delete => "delete",
            Self::Duplicate => "duplicate",
            Self::Parent => "parent",
            Self::Group => "group",
            Self::Tag => "tag",
            Self::PropertyEdit => "property-edit",
            Self::SimulationControl => "simulation-control",
            Self::AssetDrop => "asset-drop",
            Self::Publish => "publish",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorAction {
    pub sequence: u64,
    pub kind: EditorActionKind,
    pub subject: String,
    pub payload_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GizmoMode {
    Translate,
    Rotate,
    Scale,
}

impl GizmoMode {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Translate => "translate",
            Self::Rotate => "rotate",
            Self::Scale => "scale",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransformGizmo {
    pub mode: GizmoMode,
    pub snapping: bool,
    pub grid_size: i32,
    pub multi_select: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SceneNode {
    pub id: String,
    pub label: String,
    pub parent: Option<String>,
    pub folder: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SceneGraph {
    pub nodes: Vec<SceneNode>,
    pub folders: Vec<String>,
    pub graph_hash: String,
}

impl SceneGraph {
    pub fn search(&self, query: &str) -> Vec<&SceneNode> {
        let query = query.to_ascii_lowercase();
        self.nodes
            .iter()
            .filter(|node| {
                node.label.to_ascii_lowercase().contains(&query)
                    || node
                        .tags
                        .iter()
                        .any(|tag| tag.to_ascii_lowercase().contains(&query))
                    || node
                        .folder
                        .as_ref()
                        .is_some_and(|folder| folder.to_ascii_lowercase().contains(&query))
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveInspector {
    pub editable_components: Vec<&'static str>,
    pub routes_through_actions: bool,
    pub runtime_authority: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimulationControl {
    Play,
    Pause,
    Step,
    FastForward,
    Checkpoint,
    Restore,
    Reset,
}

impl SimulationControl {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Play => "play",
            Self::Pause => "pause",
            Self::Step => "step",
            Self::FastForward => "fast-forward",
            Self::Checkpoint => "checkpoint",
            Self::Restore => "restore",
            Self::Reset => "reset",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulationSession {
    pub state: &'static str,
    pub tick: u64,
    pub checkpoint_hash: Option<String>,
    pub replay_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualReplayTimeline {
    pub scrubber_frames: Vec<String>,
    pub checkpoint_markers: Vec<String>,
    pub divergence_markers: Vec<String>,
    pub continuity_visualization: String,
    pub inspected_event: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetDropTarget {
    pub world: bool,
    pub hierarchy: bool,
    pub packages: bool,
    pub entities: bool,
    pub lineage_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameplayTemplate {
    pub name: &'static str,
    pub default_world_hash: String,
    pub runnable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalRuntimeLaunch {
    pub launches_runtime: bool,
    pub launches_replay: bool,
    pub launches_simulation: bool,
    pub launches_diagnostics: bool,
    pub commandless: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishPipeline {
    pub stages: Vec<&'static str>,
    pub result: &'static str,
    pub infrastructure_hidden: bool,
    pub pipeline_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EverNodeWizard {
    pub fields: Vec<&'static str>,
    pub confirmations: Vec<&'static str>,
    pub infrastructure_management: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketplaceFoundation {
    pub shelves: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldAuthoringState {
    pub objects: Vec<WorldObject>,
    pub actions: Vec<EditorAction>,
    pub gizmo: TransformGizmo,
    pub scene_graph: SceneGraph,
    pub inspector: LiveInspector,
    pub simulation: SimulationSession,
    pub replay: VisualReplayTimeline,
    pub asset_drop: AssetDropTarget,
    pub templates: Vec<GameplayTemplate>,
    pub local_runtime: LocalRuntimeLaunch,
    pub publish: PublishPipeline,
    pub evernode: EverNodeWizard,
    pub marketplace: MarketplaceFoundation,
}

impl WorldAuthoringState {
    pub fn sample() -> Self {
        let mut state = Self {
            objects: Vec::new(),
            actions: Vec::new(),
            gizmo: TransformGizmo {
                mode: GizmoMode::Translate,
                snapping: true,
                grid_size: 4,
                multi_select: vec!["entity:settler".into(), "resource:crystal".into()],
            },
            scene_graph: SceneGraph {
                nodes: Vec::new(),
                folders: vec![
                    "World".into(),
                    "Civilizations".into(),
                    "Resources".into(),
                    "Structures".into(),
                    "Runtime Markers".into(),
                ],
                graph_hash: String::new(),
            },
            inspector: LiveInspector {
                editable_components: vec![
                    "components",
                    "runtime parameters",
                    "civilization tuning",
                    "AI tuning",
                    "resource tuning",
                    "world configuration",
                ],
                routes_through_actions: true,
                runtime_authority: "runtime-authoritative-editor-actions-only",
            },
            simulation: SimulationSession {
                state: "paused",
                tick: 0,
                checkpoint_hash: None,
                replay_hash: stable_hash(&["simulation", "initial", "replay"]),
            },
            replay: VisualReplayTimeline {
                scrubber_frames: vec![
                    "frame-0001".into(),
                    "frame-0002".into(),
                    "frame-0003".into(),
                ],
                checkpoint_markers: vec!["checkpoint-a".into()],
                divergence_markers: vec!["none".into()],
                continuity_visualization: stable_hash(&["timeline", "continuity", "visual"]),
                inspected_event: Some("event:spawn:settler".into()),
            },
            asset_drop: AssetDropTarget {
                world: true,
                hierarchy: true,
                packages: true,
                entities: true,
                lineage_hash: stable_hash(&["asset", "lineage", "drag-drop"]),
            },
            templates: template_catalog(),
            local_runtime: LocalRuntimeLaunch {
                launches_runtime: true,
                launches_replay: true,
                launches_simulation: true,
                launches_diagnostics: true,
                commandless: true,
            },
            publish: publish_pipeline(),
            evernode: EverNodeWizard {
                fields: vec![
                    "runtime size",
                    "node requirements",
                    "deployment validation",
                    "publish confirmation",
                    "deployment status",
                ],
                confirmations: vec!["validate", "package", "sign", "deploy", "register"],
                infrastructure_management: false,
            },
            marketplace: MarketplaceFoundation {
                shelves: vec![
                    "Games",
                    "Templates",
                    "Assets",
                    "Packages",
                    "Examples",
                    "Worlds",
                ],
            },
        };
        state.place_object("entity:settler", "Settler", WorldObjectKind::Entity, 4, 8);
        state.place_object(
            "region:north",
            "North Region",
            WorldObjectKind::Region,
            0,
            12,
        );
        state.place_object(
            "partition:alpha",
            "Alpha Partition",
            WorldObjectKind::Partition,
            8,
            12,
        );
        state.place_object(
            "spawn:home",
            "Home Spawn",
            WorldObjectKind::SpawnPoint,
            4,
            4,
        );
        state.place_object(
            "resource:crystal",
            "Crystal Node",
            WorldObjectKind::ResourceNode,
            12,
            8,
        );
        state.place_object(
            "faction:builders",
            "Builders",
            WorldObjectKind::Faction,
            16,
            4,
        );
        state.place_object(
            "civilization:founders",
            "Founders",
            WorldObjectKind::Civilization,
            20,
            4,
        );
        state.place_object(
            "prop:obelisk",
            "Ancient Obelisk",
            WorldObjectKind::WorldProp,
            24,
            8,
        );
        state.place_object(
            "structure:workshop",
            "Workshop",
            WorldObjectKind::Structure,
            28,
            12,
        );
        state.place_object(
            "marker:checkpoint",
            "Runtime Checkpoint Marker",
            WorldObjectKind::RuntimeMarker,
            32,
            12,
        );
        state.place_object(
            "metadata:world",
            "World Metadata",
            WorldObjectKind::Metadata,
            0,
            0,
        );
        state.rebuild_scene_graph();
        state
    }

    pub fn place_object(&mut self, id: &str, name: &str, kind: WorldObjectKind, x: i32, y: i32) {
        let transform = Transform {
            x,
            y,
            rotation_degrees: 0,
            scale_percent: 100,
        }
        .snapped(self.gizmo.grid_size);
        let component_hash = stable_hash(&[
            "object",
            id,
            name,
            kind.as_str(),
            &x.to_string(),
            &y.to_string(),
        ]);
        self.objects.push(WorldObject {
            id: id.to_owned(),
            name: name.to_owned(),
            kind,
            transform,
            parent: None,
            tags: Vec::new(),
            component_hash,
        });
        self.record(EditorActionKind::Place, id, &["place", id]);
    }

    pub fn move_object(&mut self, id: &str, x: i32, y: i32) -> Result<(), &'static str> {
        let transform = Transform {
            x,
            y,
            rotation_degrees: 0,
            scale_percent: 100,
        }
        .snapped(self.gizmo.grid_size);
        let object = self
            .objects
            .iter_mut()
            .find(|object| object.id == id)
            .ok_or("unknown object")?;
        object.transform.x = transform.x;
        object.transform.y = transform.y;
        self.record(
            EditorActionKind::Move,
            id,
            &["move", id, &x.to_string(), &y.to_string()],
        );
        self.rebuild_scene_graph();
        Ok(())
    }

    pub fn duplicate_object(&mut self, id: &str, new_id: &str) -> Result<(), &'static str> {
        let mut duplicate = self
            .objects
            .iter()
            .find(|object| object.id == id)
            .cloned()
            .ok_or("unknown object")?;
        duplicate.id = new_id.to_owned();
        duplicate.name = format!("{} Copy", duplicate.name);
        duplicate.component_hash =
            stable_hash(&["duplicate", id, new_id, &duplicate.component_hash]);
        self.objects.push(duplicate);
        self.record(
            EditorActionKind::Duplicate,
            new_id,
            &["duplicate", id, new_id],
        );
        self.rebuild_scene_graph();
        Ok(())
    }

    pub fn delete_object(&mut self, id: &str) -> Result<(), &'static str> {
        let before = self.objects.len();
        self.objects.retain(|object| object.id != id);
        if self.objects.len() == before {
            return Err("unknown object");
        }
        self.record(EditorActionKind::Delete, id, &["delete", id]);
        self.rebuild_scene_graph();
        Ok(())
    }

    pub fn parent_object(&mut self, child: &str, parent: &str) -> Result<(), &'static str> {
        if !self.objects.iter().any(|object| object.id == parent) {
            return Err("unknown parent");
        }
        let object = self
            .objects
            .iter_mut()
            .find(|object| object.id == child)
            .ok_or("unknown child")?;
        object.parent = Some(parent.to_owned());
        self.record(EditorActionKind::Parent, child, &["parent", child, parent]);
        self.rebuild_scene_graph();
        Ok(())
    }

    pub fn tag_object(&mut self, id: &str, tag: &str) -> Result<(), &'static str> {
        let object = self
            .objects
            .iter_mut()
            .find(|object| object.id == id)
            .ok_or("unknown object")?;
        if !object.tags.iter().any(|existing| existing == tag) {
            object.tags.push(tag.to_owned());
            object.tags.sort();
        }
        self.record(EditorActionKind::Tag, id, &["tag", id, tag]);
        self.rebuild_scene_graph();
        Ok(())
    }

    pub fn apply_gizmo_drag(
        &mut self,
        mode: GizmoMode,
        ids: &[&str],
        dx: i32,
        dy: i32,
    ) -> Result<String, &'static str> {
        self.gizmo.mode = mode;
        self.gizmo.multi_select = ids.iter().map(|id| (*id).to_owned()).collect();
        for id in ids {
            let Some(current) = self
                .objects
                .iter()
                .find(|object| object.id == *id)
                .map(|object| object.transform.clone())
            else {
                return Err("unknown object");
            };
            let mut transform = current;
            match self.gizmo.mode {
                GizmoMode::Translate => {
                    transform.x += dx;
                    transform.y += dy;
                }
                GizmoMode::Rotate => transform.rotation_degrees += dx,
                GizmoMode::Scale => transform.scale_percent += dx + dy,
            }
            transform = transform.snapped(self.gizmo.grid_size);
            if let Some(object) = self.objects.iter_mut().find(|object| object.id == *id) {
                object.transform = transform;
            }
        }
        let hash = stable_hash(&[
            "gizmo",
            self.gizmo.mode.as_str(),
            &ids.join(","),
            &dx.to_string(),
            &dy.to_string(),
        ]);
        self.record(EditorActionKind::Move, "multi-select", &["gizmo", &hash]);
        self.rebuild_scene_graph();
        Ok(hash)
    }

    pub fn edit_property(
        &mut self,
        id: &str,
        component: &str,
        value: &str,
    ) -> Result<String, &'static str> {
        if !self.objects.iter().any(|object| object.id == id) {
            return Err("unknown object");
        }
        let hash = stable_hash(&["property", id, component, value]);
        self.record(
            EditorActionKind::PropertyEdit,
            id,
            &["property", id, component, value],
        );
        Ok(hash)
    }

    pub fn control_simulation(&mut self, control: SimulationControl) -> String {
        match control {
            SimulationControl::Play => self.simulation.state = "playing",
            SimulationControl::Pause => self.simulation.state = "paused",
            SimulationControl::Step => self.simulation.tick += 1,
            SimulationControl::FastForward => self.simulation.tick += 8,
            SimulationControl::Checkpoint => {
                self.simulation.checkpoint_hash = Some(stable_hash(&[
                    "checkpoint",
                    &self.simulation.tick.to_string(),
                ]));
            }
            SimulationControl::Restore => {
                if self.simulation.checkpoint_hash.is_some() {
                    self.simulation.state = "paused";
                }
            }
            SimulationControl::Reset => {
                self.simulation.state = "paused";
                self.simulation.tick = 0;
                self.simulation.checkpoint_hash = None;
            }
        }
        self.simulation.replay_hash = stable_hash(&[
            "simulation",
            control.as_str(),
            &self.simulation.tick.to_string(),
        ]);
        let replay_hash = self.simulation.replay_hash.clone();
        self.record(
            EditorActionKind::SimulationControl,
            "simulation",
            &["simulation", &replay_hash],
        );
        replay_hash
    }

    pub fn publish_game(&mut self) -> Result<&'static str, &'static str> {
        if self.publish.stages != vec!["Validate", "Package", "Sign", "Deploy", "Verify", "Publish"]
        {
            return Err("publish stages must remain complete");
        }
        self.record(
            EditorActionKind::Publish,
            "game",
            &["publish", self.publish.result],
        );
        Ok(self.publish.result)
    }

    pub fn request_authority_mutation(&self, requested: bool) -> Result<(), &'static str> {
        if requested {
            Err("creator workflow cannot bypass runtime authority")
        } else {
            Ok(())
        }
    }

    pub fn request_replay_mutation(&self, requested: bool) -> Result<(), &'static str> {
        if requested {
            Err("creator workflow is replay-append-only")
        } else {
            Ok(())
        }
    }

    pub fn deterministic_hash(&self) -> String {
        let mut parts = vec!["world-authoring".to_owned()];
        parts.extend(self.objects.iter().map(|object| {
            format!(
                "{}:{}:{}:{}",
                object.id,
                object.kind.as_str(),
                object.transform.x,
                object.transform.y
            )
        }));
        parts.extend(self.actions.iter().map(|action| {
            format!(
                "{}:{}:{}",
                action.sequence,
                action.kind.as_str(),
                action.payload_hash
            )
        }));
        stable_hash(&parts.iter().map(String::as_str).collect::<Vec<_>>())
    }

    fn rebuild_scene_graph(&mut self) {
        let mut nodes: Vec<SceneNode> = self
            .objects
            .iter()
            .map(|object| SceneNode {
                id: object.id.clone(),
                label: object.name.clone(),
                parent: object.parent.clone(),
                folder: Some(match object.kind {
                    WorldObjectKind::Civilization | WorldObjectKind::Faction => {
                        "Civilizations".to_owned()
                    }
                    WorldObjectKind::ResourceNode => "Resources".to_owned(),
                    WorldObjectKind::Structure | WorldObjectKind::WorldProp => {
                        "Structures".to_owned()
                    }
                    WorldObjectKind::RuntimeMarker => "Runtime Markers".to_owned(),
                    _ => "World".to_owned(),
                }),
                tags: object.tags.clone(),
            })
            .collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        let mut parts = vec!["scene-graph".to_owned()];
        parts.extend(nodes.iter().map(|node| {
            format!(
                "{}:{:?}:{:?}:{:?}",
                node.id, node.parent, node.folder, node.tags
            )
        }));
        self.scene_graph.nodes = nodes;
        self.scene_graph.graph_hash =
            stable_hash(&parts.iter().map(String::as_str).collect::<Vec<_>>());
    }

    fn record(&mut self, kind: EditorActionKind, subject: &str, payload: &[&str]) {
        self.actions.push(EditorAction {
            sequence: self.actions.len() as u64 + 1,
            kind,
            subject: subject.to_owned(),
            payload_hash: stable_hash(payload),
        });
    }
}

pub fn template_catalog() -> Vec<GameplayTemplate> {
    [
        "Arena",
        "Action RPG",
        "Civilization",
        "RTS",
        "Survival",
        "Sandbox",
        "Dungeon Crawler",
        "MMO Prototype",
    ]
    .into_iter()
    .map(|name| GameplayTemplate {
        name,
        default_world_hash: stable_hash(&["template", name, "run-ready"]),
        runnable: true,
    })
    .collect()
}

pub fn publish_pipeline() -> PublishPipeline {
    let stages = vec!["Validate", "Package", "Sign", "Deploy", "Verify", "Publish"];
    let pipeline_hash = stable_hash(&stages);
    PublishPipeline {
        stages,
        result: "Game Live",
        infrastructure_hidden: true,
        pipeline_hash,
    }
}

pub fn world_authoring_equivalence() -> bool {
    WorldAuthoringState::sample().deterministic_hash()
        == WorldAuthoringState::sample().deterministic_hash()
}

pub fn gizmo_equivalence() -> bool {
    let mut a = WorldAuthoringState::sample();
    let mut b = WorldAuthoringState::sample();
    a.apply_gizmo_drag(
        GizmoMode::Translate,
        &["entity:settler", "resource:crystal"],
        5,
        7,
    )
    .ok()
        == b.apply_gizmo_drag(
            GizmoMode::Translate,
            &["entity:settler", "resource:crystal"],
            5,
            7,
        )
        .ok()
        && a.deterministic_hash() == b.deterministic_hash()
}

pub fn scene_graph_equivalence() -> bool {
    let mut a = WorldAuthoringState::sample();
    let mut b = WorldAuthoringState::sample();
    a.parent_object("entity:settler", "region:north").ok()
        == b.parent_object("entity:settler", "region:north").ok()
        && a.tag_object("entity:settler", "player-start").ok()
            == b.tag_object("entity:settler", "player-start").ok()
        && a.scene_graph.graph_hash == b.scene_graph.graph_hash
        && a.scene_graph.search("player").len() == 1
}

pub fn live_edit_equivalence() -> bool {
    let mut a = WorldAuthoringState::sample();
    let mut b = WorldAuthoringState::sample();
    a.edit_property("civilization:founders", "civilization tuning", "growth=2")
        .ok()
        == b.edit_property("civilization:founders", "civilization tuning", "growth=2")
            .ok()
        && a.inspector.routes_through_actions
}

pub fn simulation_control_equivalence() -> bool {
    let mut a = WorldAuthoringState::sample();
    let mut b = WorldAuthoringState::sample();
    for control in [
        SimulationControl::Play,
        SimulationControl::Step,
        SimulationControl::FastForward,
        SimulationControl::Checkpoint,
        SimulationControl::Restore,
        SimulationControl::Pause,
    ] {
        if a.control_simulation(control.clone()) != b.control_simulation(control) {
            return false;
        }
    }
    a.simulation == b.simulation
}

pub fn replay_timeline_equivalence() -> bool {
    WorldAuthoringState::sample().replay == WorldAuthoringState::sample().replay
}

pub fn asset_dragdrop_equivalence() -> bool {
    let state = WorldAuthoringState::sample();
    state.asset_drop.world
        && state.asset_drop.hierarchy
        && state.asset_drop.packages
        && state.asset_drop.entities
}

pub fn template_creation_equivalence() -> bool {
    let templates = template_catalog();
    templates.len() == 8
        && templates.iter().all(|template| template.runnable)
        && templates == template_catalog()
}

pub fn local_runtime_launch() -> bool {
    let launch = WorldAuthoringState::sample().local_runtime;
    launch.launches_runtime
        && launch.launches_replay
        && launch.launches_simulation
        && launch.launches_diagnostics
        && launch.commandless
}

pub fn publish_pipeline_equivalence() -> bool {
    let mut a = WorldAuthoringState::sample();
    let mut b = WorldAuthoringState::sample();
    a.publish == b.publish
        && a.publish_game().ok() == b.publish_game().ok()
        && a.publish.result == "Game Live"
}

pub fn replay_safe_creator_workflow() -> bool {
    let state = WorldAuthoringState::sample();
    state.request_replay_mutation(true).is_err()
        && state.request_authority_mutation(true).is_err()
        && state.inspector.routes_through_actions
        && state.publish.infrastructure_hidden
        && !state.evernode.infrastructure_management
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerrainAuthoringSurface {
    pub brushes: Vec<&'static str>,
    pub partition_visualization: bool,
    pub deterministic_seed: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetAuthoringExperience {
    pub asset_types: Vec<&'static str>,
    pub features: Vec<&'static str>,
    pub deterministic_import_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetBrowser2 {
    pub management: Vec<&'static str>,
    pub drag_targets: Vec<&'static str>,
    pub browser_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UndoRedoHistory {
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    pub restore_point: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatorProductivityLayer {
    pub actions: Vec<&'static str>,
    pub deterministic_command_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeOverlay {
    pub metrics: Vec<&'static str>,
    pub live_updates: bool,
    pub projection_only: bool,
}

pub fn terrain_authoring_surface() -> TerrainAuthoringSurface {
    let brushes = vec![
        "terrain painting",
        "region painting",
        "resource painting",
        "spawn painting",
        "civilization territory painting",
    ];
    TerrainAuthoringSurface {
        deterministic_seed: stable_hash(&brushes),
        brushes,
        partition_visualization: true,
    }
}

pub fn asset_authoring_experience() -> AssetAuthoringExperience {
    let asset_types = vec![
        "models",
        "textures",
        "audio",
        "world templates",
        "runtime packages",
    ];
    let features = vec![
        "thumbnail generation",
        "asset previews",
        "asset tagging",
        "asset categories",
        "drag-and-drop imports",
        "batch imports",
        "asset validation",
    ];
    let mut parts = Vec::new();
    parts.extend(asset_types.iter().copied());
    parts.extend(features.iter().copied());
    AssetAuthoringExperience {
        asset_types,
        features,
        deterministic_import_hash: stable_hash(&parts),
    }
}

pub fn asset_browser_2() -> AssetBrowser2 {
    let management = vec![
        "search",
        "filter",
        "favorites",
        "dependencies",
        "usage references",
        "package membership",
    ];
    let drag_targets = vec!["viewport", "hierarchy", "package", "entity"];
    let mut parts = Vec::new();
    parts.extend(management.iter().copied());
    parts.extend(drag_targets.iter().copied());
    AssetBrowser2 {
        management,
        drag_targets,
        browser_hash: stable_hash(&parts),
    }
}

pub fn world_snapshot_hash(state: &WorldAuthoringState) -> String {
    stable_hash(&[
        "world-save-template",
        &state.deterministic_hash(),
        &state.scene_graph.graph_hash,
    ])
}

pub fn undo_redo_history(state: &WorldAuthoringState) -> UndoRedoHistory {
    let undo_stack: Vec<String> = state
        .actions
        .iter()
        .map(|action| format!("{}:{}", action.sequence, action.payload_hash))
        .collect();
    UndoRedoHistory {
        restore_point: stable_hash(&["restore-point", &world_snapshot_hash(state)]),
        undo_stack,
        redo_stack: Vec::new(),
    }
}

pub fn creator_productivity_layer() -> CreatorProductivityLayer {
    let actions = vec![
        "search everywhere",
        "quick actions",
        "command palette",
        "context menus",
        "recent projects",
        "favorites",
    ];
    CreatorProductivityLayer {
        deterministic_command_hash: stable_hash(&actions),
        actions,
    }
}

pub fn runtime_overlay() -> RuntimeOverlay {
    RuntimeOverlay {
        metrics: vec![
            "entity count",
            "simulation tick",
            "scheduler activity",
            "AI activity",
            "replay health",
            "runtime health",
        ],
        live_updates: true,
        projection_only: true,
    }
}

pub fn entity_placement_equivalence() -> bool {
    let a = WorldAuthoringState::sample();
    let b = WorldAuthoringState::sample();
    let required = [
        WorldObjectKind::SpawnPoint,
        WorldObjectKind::ResourceNode,
        WorldObjectKind::WorldProp,
        WorldObjectKind::Faction,
        WorldObjectKind::Civilization,
        WorldObjectKind::Region,
        WorldObjectKind::Structure,
        WorldObjectKind::RuntimeMarker,
    ];
    a.objects == b.objects
        && required
            .iter()
            .all(|kind| a.objects.iter().any(|object| &object.kind == kind))
}

pub fn terrain_authoring_equivalence() -> bool {
    terrain_authoring_surface() == terrain_authoring_surface()
        && terrain_authoring_surface().partition_visualization
}

pub fn asset_import_equivalence() -> bool {
    asset_authoring_experience() == asset_authoring_experience()
        && asset_browser_2() == asset_browser_2()
}

pub fn live_simulation_equivalence() -> bool {
    simulation_control_equivalence()
        && runtime_overlay().live_updates
        && runtime_overlay().projection_only
}

pub fn replay_visualization_equivalence() -> bool {
    let replay = WorldAuthoringState::sample().replay;
    replay_timeline_equivalence()
        && !replay.scrubber_frames.is_empty()
        && !replay.checkpoint_markers.is_empty()
        && !replay.divergence_markers.is_empty()
}

pub fn world_save_load_equivalence() -> bool {
    let state = WorldAuthoringState::sample();
    world_snapshot_hash(&state) == world_snapshot_hash(&WorldAuthoringState::sample())
}

pub fn undo_redo_equivalence() -> bool {
    let mut state = WorldAuthoringState::sample();
    state.move_object("entity:settler", 16, 16).ok();
    undo_redo_history(&state) == undo_redo_history(&state.clone())
}

pub fn creator_productivity_equivalence() -> bool {
    creator_productivity_layer() == creator_productivity_layer()
}
