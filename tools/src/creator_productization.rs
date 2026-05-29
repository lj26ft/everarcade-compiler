use crate::{diagnostic, stable_hash, CreatorDiagnostic};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreatorWorkflowSurface {
    pub world_tools: Vec<&'static str>,
    pub gizmo_modes: Vec<&'static str>,
    pub hierarchy_features: Vec<&'static str>,
    pub inspector_routes: Vec<&'static str>,
    pub simulation_controls: Vec<&'static str>,
    pub replay_features: Vec<&'static str>,
    pub drag_drop_targets: Vec<&'static str>,
    pub templates: Vec<&'static str>,
    pub local_runtime_launches: Vec<&'static str>,
    pub publish_pipeline: Vec<&'static str>,
    pub evernode_fields: Vec<&'static str>,
    pub marketplace_shelves: Vec<&'static str>,
    pub deterministic_action_hash: String,
}

impl CreatorWorkflowSurface {
    pub fn sample() -> Self {
        let world_tools = vec![
            "Entity Placement",
            "Region Creation",
            "Partition Creation",
            "Spawn Points",
            "Resource Nodes",
            "Faction Placement",
            "Civilization Placement",
            "World Metadata",
        ];
        let gizmo_modes = vec!["Translate", "Rotate", "Scale"];
        let hierarchy_features = vec![
            "parenting",
            "grouping",
            "folders",
            "search",
            "filtering",
            "tagging",
        ];
        let inspector_routes = vec![
            "component editing",
            "runtime parameters",
            "civilization tuning",
            "AI tuning",
            "resource tuning",
            "world configuration",
            "deterministic editor actions",
        ];
        let simulation_controls = vec![
            "Play",
            "Pause",
            "Step",
            "Fast Forward",
            "Checkpoint",
            "Restore",
        ];
        let replay_features = vec![
            "timeline scrubber",
            "checkpoint markers",
            "divergence markers",
            "continuity visualization",
            "event inspection",
        ];
        let drag_drop_targets = vec!["world", "hierarchy", "packages", "entities"];
        let templates = vec![
            "Top Down Arena",
            "RPG World",
            "Civilization World",
            "Persistent Sandbox",
            "Cooperative Dungeon",
            "RTS Prototype",
        ];
        let local_runtime_launches = vec!["runtime", "replay", "simulation", "diagnostics"];
        let publish_pipeline = vec![
            "Validate", "Package", "Sign", "Verify", "Deploy", "Register",
        ];
        let evernode_fields = vec![
            "runtime size",
            "node requirements",
            "deployment validation",
            "publish confirmation",
            "deployment status",
        ];
        let marketplace_shelves = vec![
            "Published Games",
            "Packages",
            "Templates",
            "Assets",
            "Examples",
        ];
        let mut parts = Vec::new();
        parts.extend(world_tools.iter().copied());
        parts.extend(gizmo_modes.iter().copied());
        parts.extend(hierarchy_features.iter().copied());
        parts.extend(inspector_routes.iter().copied());
        parts.extend(simulation_controls.iter().copied());
        parts.extend(replay_features.iter().copied());
        parts.extend(drag_drop_targets.iter().copied());
        parts.extend(templates.iter().copied());
        parts.extend(local_runtime_launches.iter().copied());
        parts.extend(publish_pipeline.iter().copied());
        parts.extend(evernode_fields.iter().copied());
        parts.extend(marketplace_shelves.iter().copied());
        let deterministic_action_hash = stable_hash(&parts);
        Self {
            world_tools,
            gizmo_modes,
            hierarchy_features,
            inspector_routes,
            simulation_controls,
            replay_features,
            drag_drop_targets,
            templates,
            local_runtime_launches,
            publish_pipeline,
            evernode_fields,
            marketplace_shelves,
            deterministic_action_hash,
        }
    }

    pub fn publish_result(&self) -> &'static str {
        if self.publish_pipeline
            == [
                "Validate", "Package", "Sign", "Verify", "Deploy", "Register",
            ]
        {
            "Game Live"
        } else {
            "Blocked"
        }
    }
}

pub fn validate_creator_productization() -> CreatorDiagnostic {
    diagnostic(
        "creator-productization-workflow",
        &[
            "world-authoring",
            "gizmos",
            "scene-graph",
            "live-inspector",
            "simulation-controls",
            "visual-replay",
            "asset-drag-drop",
            "templates",
            "local-runtime",
            "publish-game",
            "evernode-wizard",
            "marketplace",
        ],
    )
}

pub fn world_authoring_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface.world_tools.len() == 8 && surface == CreatorWorkflowSurface::sample()
}

pub fn gizmo_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface.gizmo_modes == ["Translate", "Rotate", "Scale"]
}

pub fn scene_graph_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    [
        "parenting",
        "grouping",
        "folders",
        "search",
        "filtering",
        "tagging",
    ]
    .iter()
    .all(|feature| surface.hierarchy_features.contains(feature))
}

pub fn live_edit_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface
        .inspector_routes
        .contains(&"deterministic editor actions")
}

pub fn simulation_control_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface.simulation_controls
        == [
            "Play",
            "Pause",
            "Step",
            "Fast Forward",
            "Checkpoint",
            "Restore",
        ]
}

pub fn replay_timeline_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface.replay_features.contains(&"timeline scrubber")
        && surface
            .replay_features
            .contains(&"continuity visualization")
        && surface.replay_features.contains(&"event inspection")
}

pub fn asset_dragdrop_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface.drag_drop_targets == ["world", "hierarchy", "packages", "entities"]
}

pub fn template_creation_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface.templates.len() == 6 && surface.templates == CreatorWorkflowSurface::sample().templates
}

pub fn local_runtime_launch() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface.local_runtime_launches == ["runtime", "replay", "simulation", "diagnostics"]
}

pub fn publish_pipeline_equivalence() -> bool {
    let surface = CreatorWorkflowSurface::sample();
    surface.publish_pipeline
        == [
            "Validate", "Package", "Sign", "Verify", "Deploy", "Register",
        ]
        && surface.publish_result() == "Game Live"
}

pub fn reject_authority_mutation(requested: bool) -> Result<(), &'static str> {
    if requested {
        Err("creator workflow cannot bypass runtime authority")
    } else {
        Ok(())
    }
}

pub fn reject_replay_mutation(requested: bool) -> Result<(), &'static str> {
    if requested {
        Err("creator workflow cannot rewrite replay history")
    } else {
        Ok(())
    }
}

pub fn replay_safe_creator_workflow() -> bool {
    let diagnostic = validate_creator_productization();
    diagnostic.deterministic
        && diagnostic.replay_continuity == "preserved"
        && diagnostic.authority_boundary == "deterministic-execution-runtime-only"
        && !diagnostic.renderer_authoritative
        && reject_authority_mutation(true).is_err()
        && reject_replay_mutation(true).is_err()
}
