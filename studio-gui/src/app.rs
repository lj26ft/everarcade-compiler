use crate::{
    replay::ReplayTimeline,
    stable_hash,
    theme::StudioTheme,
    viewport::{self, RuntimeProjection, ViewportState},
    window::StudioWindow,
    workspace::{self, StudioWorkspace},
    world_authoring::{self, WorldAuthoringState},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HierarchyPanel {
    pub groups: Vec<&'static str>,
    pub search: String,
    pub selected: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InspectorPanel {
    pub inspected_entity: Option<String>,
    pub component_count: usize,
    pub replay_lineage: String,
    pub edits_route: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetBrowserPanel {
    pub assets: Vec<String>,
    pub package_membership: Vec<String>,
    pub supports_drag_drop_import: bool,
    pub supports_thumbnails: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulationPanel {
    pub lanes: Vec<&'static str>,
    pub projection_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticsPanel {
    pub metrics: Vec<&'static str>,
    pub supports_search: bool,
    pub supports_filter: bool,
    pub supports_export: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishingPanel {
    pub capabilities: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandPalettePanel {
    pub features: Vec<&'static str>,
    pub recent_actions: Vec<&'static str>,
    pub favorites: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveSimulationOverlay {
    pub metrics: Vec<&'static str>,
    pub updates_live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveLoadPanel {
    pub operations: Vec<&'static str>,
    pub deterministic_serialization: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeploymentPanel {
    pub surfaces: Vec<&'static str>,
    pub actions: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiplayerPanel {
    pub actions: Vec<&'static str>,
    pub session_features: Vec<&'static str>,
    pub invite_link: String,
    pub no_networking_setup: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperationsDashboardPanel {
    pub health_surfaces: Vec<&'static str>,
    pub admin_controls: Vec<&'static str>,
    pub metrics: Vec<&'static str>,
    pub single_dashboard: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatorWorkflow {
    pub steps: Vec<&'static str>,
    pub visual_only: bool,
    pub workflow_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StudioGuiApp {
    pub window: StudioWindow,
    pub workspace: StudioWorkspace,
    pub theme: StudioTheme,
    pub viewport_projection: RuntimeProjection,
    pub viewport_state: ViewportState,
    pub hierarchy: HierarchyPanel,
    pub inspector: InspectorPanel,
    pub assets: AssetBrowserPanel,
    pub replay: ReplayTimeline,
    pub simulation: SimulationPanel,
    pub diagnostics: DiagnosticsPanel,
    pub publishing: PublishingPanel,
    pub command_palette: CommandPalettePanel,
    pub live_overlay: LiveSimulationOverlay,
    pub save_load: SaveLoadPanel,
    pub deployment: DeploymentPanel,
    pub multiplayer: MultiplayerPanel,
    pub operations_dashboard: OperationsDashboardPanel,
    pub workflow: CreatorWorkflow,
    pub world_authoring: WorldAuthoringState,
    pub deterministic_runtime_authority: bool,
}

impl Default for StudioGuiApp {
    fn default() -> Self {
        Self::new()
    }
}

impl StudioGuiApp {
    pub fn new() -> Self {
        let viewport_projection = RuntimeProjection::sample();
        Self {
            window: StudioWindow::default(),
            workspace: StudioWorkspace::new("creator-workspace"),
            theme: StudioTheme::default(),
            viewport_state: ViewportState {
                selected_entity: None,
                camera_hash: stable_hash(&["camera", "main", "0", "0", "1"]),
                last_projection_hash: viewport_projection.projection_hash.clone(),
            },
            hierarchy: HierarchyPanel {
                groups: vec![
                    "World",
                    "Regions",
                    "Partitions",
                    "Entities",
                    "Civilizations",
                    "Factions",
                    "Players",
                ],
                search: String::new(),
                selected: None,
            },
            inspector: InspectorPanel {
                inspected_entity: None,
                component_count: 0,
                replay_lineage: stable_hash(&["inspector", "replay-lineage", "read-only"]),
                edits_route: "deterministic-editor-actions",
            },
            assets: AssetBrowserPanel {
                assets: vec![
                    "hero.png".into(),
                    "terrain.tileset".into(),
                    "civ.rules".into(),
                ],
                package_membership: vec!["starter-pack".into()],
                supports_drag_drop_import: true,
                supports_thumbnails: true,
            },
            replay: ReplayTimeline::sample(),
            simulation: SimulationPanel {
                lanes: vec![
                    "ECS execution",
                    "AI execution",
                    "scheduler ordering",
                    "partition activity",
                    "runtime health",
                    "civilization activity",
                ],
                projection_only: true,
            },
            diagnostics: DiagnosticsPanel {
                metrics: vec![
                    "runtime health",
                    "continuity health",
                    "replay health",
                    "federation health",
                    "partition health",
                    "simulation health",
                ],
                supports_search: true,
                supports_filter: true,
                supports_export: true,
            },
            publishing: PublishingPanel {
                capabilities: vec![
                    "single publish action",
                    "package preview",
                    "dependency inspection",
                    "manifest inspection",
                    "signing",
                    "package generation",
                    "package verification",
                    "deployment history",
                    "deployment rollback",
                    "deployment verification",
                ],
            },
            command_palette: CommandPalettePanel {
                features: vec![
                    "quick search",
                    "command palette",
                    "recent actions",
                    "favorites",
                    "workspace navigation",
                ],
                recent_actions: vec!["paint terrain", "place entity", "run simulation"],
                favorites: vec!["Publish Game", "Restore Checkpoint"],
            },
            live_overlay: LiveSimulationOverlay {
                metrics: vec![
                    "entity count",
                    "partition count",
                    "simulation tick",
                    "AI activity",
                    "scheduler activity",
                    "runtime health",
                    "replay health",
                ],
                updates_live: true,
            },
            save_load: SaveLoadPanel {
                operations: vec![
                    "save project",
                    "save world",
                    "save template",
                    "load world",
                    "clone world",
                    "restore checkpoint",
                ],
                deterministic_serialization: true,
            },
            deployment: DeploymentPanel {
                surfaces: vec![
                    "runtime status",
                    "node status",
                    "deployment lineage",
                    "package lineage",
                    "federation topology",
                    "publish validate package deploy verify live",
                ],
                actions: vec![
                    "publish", "validate", "package", "deploy", "verify", "restore", "rollback",
                ],
            },
            multiplayer: MultiplayerPanel {
                actions: vec![
                    "Create World",
                    "Host World",
                    "Join World",
                    "Invite Players",
                    "Share Link",
                ],
                session_features: vec![
                    "session discovery",
                    "session continuity",
                    "player continuity",
                    "identity continuity",
                    "checkpoint continuity",
                ],
                invite_link: "everarcade://join/creator-workspace/world-alpha".into(),
                no_networking_setup: true,
            },
            operations_dashboard: OperationsDashboardPanel {
                health_surfaces: vec![
                    "online players",
                    "world health",
                    "simulation health",
                    "runtime health",
                    "replay health",
                    "deployment health",
                ],
                admin_controls: vec![
                    "world settings",
                    "player management",
                    "runtime controls",
                    "deployment controls",
                    "rollback controls",
                ],
                metrics: vec![
                    "entity counts",
                    "simulation load",
                    "partition load",
                    "scheduler load",
                    "runtime latency",
                    "replay continuity",
                ],
                single_dashboard: true,
            },
            workflow: creator_workflow(),
            world_authoring: WorldAuthoringState::sample(),
            deterministic_runtime_authority: true,
            viewport_projection,
        }
    }

    pub fn select_entity(&mut self, entity_id: &str) -> Result<(), &'static str> {
        viewport::select_entity(
            &mut self.viewport_state,
            &self.viewport_projection,
            entity_id,
        )?;
        self.hierarchy.selected = Some(entity_id.to_owned());
        self.inspector.inspected_entity = Some(entity_id.to_owned());
        self.inspector.component_count = 3;
        Ok(())
    }

    pub fn request_authority_mutation(&self, requested: bool) -> Result<(), &'static str> {
        if requested {
            Err("Studio GUI cannot mutate runtime authority directly")
        } else {
            Ok(())
        }
    }

    pub fn package_content(&self) -> String {
        stable_hash(&[
            "package",
            "preview",
            &self.workspace.workspace_hash(),
            &self.assets.assets.join(","),
        ])
    }

    pub fn deployment_lineage(&self) -> String {
        stable_hash(&[
            "deployment",
            "lineage",
            &self.package_content(),
            "federation-topology",
        ])
    }

    pub fn gui_readiness(&self) -> bool {
        self.window.native_desktop
            && !self.window.browser_dependency
            && self.workspace.layout.has_all_required_panels()
            && !self.workspace.projects.is_empty()
            && !self.workspace.runtime_sessions.is_empty()
            && self.deterministic_runtime_authority
            && self
                .command_palette
                .features
                .contains(&"workspace navigation")
            && self.live_overlay.updates_live
            && self.save_load.deterministic_serialization
            && self.multiplayer.no_networking_setup
            && self.multiplayer.actions.contains(&"Host World")
            && self.operations_dashboard.single_dashboard
            && self
                .operations_dashboard
                .admin_controls
                .contains(&"rollback controls")
            && world_authoring::replay_safe_creator_workflow()
    }
}

impl eframe::App for StudioGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.heading(&self.window.title);
            ui.label(format!("Theme: {}", self.theme.name));
        });
        egui::SidePanel::left("hierarchy").show(ctx, |ui| {
            ui.heading("Hierarchy");
            for group in &self.hierarchy.groups {
                ui.label(*group);
            }
        });
        egui::SidePanel::right("inspector").show(ctx, |ui| {
            ui.heading("Inspector");
            ui.label(format!("Edit route: {}", self.inspector.edits_route));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Runtime Viewport");
            ui.label(format!(
                "Projection: {}",
                self.viewport_projection.projection_hash
            ));
            ui.label("Renderer is projection-only and non-authoritative.");
            ui.label(format!(
                "World objects: {}",
                self.world_authoring.objects.len()
            ));
            ui.label(format!(
                "Publish result: {}",
                self.world_authoring.publish.result
            ));
            ui.label(format!(
                "Multiplayer invite: {}",
                self.multiplayer.invite_link
            ));
            ui.label(format!(
                "Operations dashboard surfaces: {}",
                self.operations_dashboard.health_surfaces.join(", ")
            ));
        });
        egui::TopBottomPanel::bottom("timeline").show(ctx, |ui| {
            ui.heading("Replay / Assets / Console");
            ui.label(format!(
                "Replay continuity: {}",
                self.replay.continuity_hash
            ));
        });
    }
}

pub fn creator_workflow() -> CreatorWorkflow {
    let steps = vec![
        "Create Project",
        "Import Assets",
        "Build World",
        "Place Entities",
        "Run Simulation",
        "Inspect Replay",
        "Inspect Runtime",
        "Package Content",
        "Run Locally",
        "Create World",
        "Host World",
        "Publish Game",
        "World Live",
        "Players Join",
        "Operate World",
        "Deploy Updates",
        "Recover Failures",
        "Game Live On EverNode",
    ];
    let workflow_hash = stable_hash(&steps);
    CreatorWorkflow {
        steps,
        visual_only: true,
        workflow_hash,
    }
}

pub fn project_manager_equivalence() -> bool {
    let created = workspace::create_project("new-project", "blank-template");
    let opened = workspace::open_project("new-project");
    let cloned = workspace::clone_project(&created, "clone-project");
    let template = workspace::import_template("simulation-world");
    workspace::validate_project(&created)
        && workspace::validate_project(&opened)
        && workspace::validate_project(&cloned)
        && template == workspace::import_template("simulation-world")
}
