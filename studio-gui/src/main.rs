#![allow(dead_code)]

mod app;
mod interactive_viewport;
mod layout;
mod replay;
mod theme;
mod viewport;
mod window;
mod workspace;
mod world_authoring;

use app::StudioGuiApp;

pub fn stable_hash(parts: &[&str]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "EverArcade Studio",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(StudioGuiApp::new())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_layout_equivalence() {
        let mut app = StudioGuiApp::new();
        app.workspace.save_layout();
        let saved = app.workspace.persisted_layout.clone();
        app.workspace.restore_layout();
        assert_eq!(saved, app.workspace.layout.serialize());
        assert!(app.workspace.layout.has_all_required_panels());
        assert!(app.workspace.supports_multiple_projects());
    }

    #[test]
    fn test_viewport_selection_equivalence() {
        assert!(interactive_viewport::viewport_selection_equivalence());
        assert!(interactive_viewport::camera_controls_equivalence());
        assert!(interactive_viewport::reject_authority_mutation(true).is_err());
    }

    #[test]
    fn test_viewport_projection_integrity() {
        let app = StudioGuiApp::new();
        let same = viewport::RuntimeProjection::sample();
        assert_eq!(app.viewport_projection, same);
        assert!(!app.viewport_projection.authority_mutation);
        assert!(!viewport::renderer_is_authoritative());
        assert!(viewport::reject_authority_mutation(true).is_err());
    }

    #[test]
    fn test_hierarchy_equivalence() {
        let app = StudioGuiApp::new();
        for required in [
            "World",
            "Regions",
            "Partitions",
            "Entities",
            "Civilizations",
            "Factions",
            "Players",
        ] {
            assert!(app.hierarchy.groups.contains(&required));
        }
    }

    #[test]
    fn test_inspector_replay_safety() {
        let mut app = StudioGuiApp::new();
        app.select_entity("entity:settler").unwrap();
        assert_eq!(app.inspector.edits_route, "deterministic-editor-actions");
        assert!(app.request_authority_mutation(true).is_err());
    }

    #[test]
    fn test_asset_browser_equivalence() {
        let app = StudioGuiApp::new();
        assert!(app.assets.supports_drag_drop_import);
        assert!(app.assets.supports_thumbnails);
        assert_eq!(app.package_content(), StudioGuiApp::new().package_content());
    }

    #[test]
    fn test_replay_ui_equivalence() {
        let mut replay = replay::ReplayTimeline::sample();
        let continuity = replay.continuity_hash.clone();
        assert_eq!(replay.scrub(1), Some("frame-0002"));
        assert_eq!(continuity, replay::ReplayTimeline::sample().continuity_hash);
        assert!(replay.request_replay_mutation(true).is_err());
    }

    #[test]
    fn test_simulation_visualizer_equivalence() {
        let app = StudioGuiApp::new();
        assert!(app.simulation.projection_only);
        assert!(app.simulation.lanes.contains(&"ECS execution"));
        assert!(app.simulation.lanes.contains(&"civilization activity"));
    }

    #[test]
    fn test_project_manager_equivalence() {
        assert!(app::project_manager_equivalence());
    }

    #[test]
    fn test_deployment_workspace_equivalence() {
        let app = StudioGuiApp::new();
        assert!(app.deployment.surfaces.contains(&"federation topology"));
        assert!(app.deployment.actions.contains(&"rollback"));
        assert_eq!(
            app.deployment_lineage(),
            StudioGuiApp::new().deployment_lineage()
        );
    }

    #[test]
    fn test_creator_workflow_equivalence() {
        let app = StudioGuiApp::new();
        assert!(app.workflow.visual_only);
        assert_eq!(app.workflow.steps, app::creator_workflow().steps);
        assert_eq!(
            app.workflow.workflow_hash,
            app::creator_workflow().workflow_hash
        );
    }

    #[test]
    fn test_authority_mutation_rejection() {
        let app = StudioGuiApp::new();
        assert!(app.request_authority_mutation(true).is_err());
        assert!(viewport::reject_authority_mutation(true).is_err());
    }

    #[test]
    fn test_world_authoring_equivalence() {
        assert!(world_authoring::world_authoring_equivalence());
        let mut world = world_authoring::WorldAuthoringState::sample();
        world.move_object("entity:settler", 9, 9).unwrap();
        world
            .duplicate_object("entity:settler", "entity:settler-copy")
            .unwrap();
        world.delete_object("entity:settler-copy").unwrap();
        assert!(world
            .objects
            .iter()
            .any(|object| object.id == "entity:settler"));
    }

    #[test]
    fn test_gizmo_equivalence() {
        assert!(world_authoring::gizmo_equivalence());
    }

    #[test]
    fn test_scene_graph_equivalence() {
        assert!(world_authoring::scene_graph_equivalence());
    }

    #[test]
    fn test_live_edit_equivalence() {
        assert!(world_authoring::live_edit_equivalence());
    }

    #[test]
    fn test_simulation_control_equivalence() {
        assert!(world_authoring::simulation_control_equivalence());
    }

    #[test]
    fn test_replay_timeline_equivalence() {
        assert!(world_authoring::replay_timeline_equivalence());
    }

    #[test]
    fn test_asset_dragdrop_equivalence() {
        assert!(world_authoring::asset_dragdrop_equivalence());
    }

    #[test]
    fn test_template_creation_equivalence() {
        assert!(world_authoring::template_creation_equivalence());
    }

    #[test]
    fn test_local_runtime_launch() {
        assert!(world_authoring::local_runtime_launch());
    }

    #[test]
    fn test_publish_pipeline_equivalence() {
        assert!(world_authoring::publish_pipeline_equivalence());
    }

    #[test]
    fn test_entity_placement_equivalence() {
        assert!(world_authoring::entity_placement_equivalence());
    }

    #[test]
    fn test_terrain_authoring_equivalence() {
        assert!(world_authoring::terrain_authoring_equivalence());
    }

    #[test]
    fn test_asset_import_equivalence() {
        assert!(world_authoring::asset_import_equivalence());
    }

    #[test]
    fn test_live_simulation_equivalence() {
        assert!(world_authoring::live_simulation_equivalence());
    }

    #[test]
    fn test_replay_visualization_equivalence() {
        assert!(world_authoring::replay_visualization_equivalence());
    }

    #[test]
    fn test_world_save_load_equivalence() {
        assert!(world_authoring::world_save_load_equivalence());
    }

    #[test]
    fn test_undo_redo_equivalence() {
        assert!(world_authoring::undo_redo_equivalence());
    }

    #[test]
    fn test_replay_safe_creator_workflow() {
        assert!(world_authoring::replay_safe_creator_workflow());
    }

    #[test]
    fn test_replay_safe_studio_gui() {
        let app = StudioGuiApp::new();
        assert!(app.gui_readiness());
        assert!(app.replay.reconstruction_only);
        assert!(app.replay.request_replay_mutation(true).is_err());
        assert!(
            app.diagnostics.supports_search
                && app.diagnostics.supports_filter
                && app.diagnostics.supports_export
        );
    }
}
