#![allow(dead_code)]

mod app;
mod assets;
mod component_editor;
mod editing_engine;
mod gameplay_authoring;
mod interactive_viewport;
mod layout;
mod marketplace;
mod replay;
mod rustrig_composer;
mod terrain;
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
    fn test_marketplace_browser_and_rustrig_composer() {
        let mut browser = marketplace::MarketplaceBrowser::sample();
        assert_eq!(browser.browse_packages().len(), 5);
        assert_eq!(browser.search_packages("combat").len(), 1);
        assert!(browser.install_package("arena-vanguard-combat"));
        assert!(browser.update_package("arena-vanguard-combat"));
        assert_eq!(
            browser.view_validation_status("arena-vanguard-combat"),
            Some("certified".to_owned())
        );
        assert!(browser.remove_package("arena-vanguard-combat"));

        let mut composer = rustrig_composer::RustrigComposer::default();
        composer.drag_package("arena-vanguard-combat");
        composer.drag_package("arena-vanguard-inventory");
        assert!(composer.connect_rustrigs("arena-vanguard-combat", "arena-vanguard-inventory"));
        assert_eq!(composer.visual_composition().len(), 2);
        assert!(composer
            .record_inspection("arena-vanguard-combat")
            .is_some());
        assert!(composer
            .dependency_inspection("arena-vanguard-combat")
            .is_some());
    }

    #[test]
    fn test_interactive_editing_equivalence() {
        assert!(editing_engine::validation::interactive_editing_equivalence());
    }

    #[test]
    fn test_terrain_sculpting_equivalence() {
        assert!(terrain::terrain_sculpting_equivalence());
    }

    #[test]
    fn test_region_painting_equivalence() {
        assert!(terrain::region_painting_equivalence());
    }

    #[test]
    fn test_entity_dragdrop_equivalence() {
        assert!(world_authoring::asset_dragdrop_equivalence());
        assert!(world_authoring::entity_placement_equivalence());
    }

    #[test]
    fn test_component_editing_equivalence() {
        assert!(component_editor::component_editing_equivalence());
    }

    #[test]
    fn test_asset_preview_equivalence() {
        assert!(assets::previews::asset_preview_equivalence());
    }

    #[test]
    fn test_live_playmode_equivalence() {
        assert!(world_authoring::simulation_control_equivalence());
        assert!(world_authoring::local_runtime_launch());
    }

    #[test]
    fn test_rustrig_browser_and_composition() {
        assert!(gameplay_authoring::rustrig_visual_logic_integration());
    }

    #[test]
    fn test_creator_production_workflow() {
        let app = StudioGuiApp::new();
        assert!(app.workflow.visual_only);
        assert!(app.gui_readiness());
        assert!(world_authoring::publish_pipeline_equivalence());
    }

    #[test]
    fn test_replay_safe_editor_behavior() {
        assert!(editing_engine::validation::replay_safe_editor_behavior());
    }

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
        assert!(editing_engine::validation::undo_redo_equivalence());
    }

    #[test]
    fn test_replay_safe_creator_workflow() {
        assert!(world_authoring::replay_safe_creator_workflow());
    }

    #[test]
    fn test_gameplay_authoring_framework_equivalence() {
        let app = StudioGuiApp::new();
        assert!(gameplay_authoring::visual_logic_equivalence());
        assert!(gameplay_authoring::gameplay_event_framework_equivalence());
        assert!(gameplay_authoring::creator_content_systems_complete());
        assert!(gameplay_authoring::end_to_end_creator_flow_equivalence());
        assert!(app
            .gameplay_authoring
            .can_create_gameplay_without_infrastructure_code());
        assert!(app.workflow.steps.contains(&"Create Gameplay"));
        assert!(app.workflow.steps.contains(&"Create UI"));
        assert!(app.workflow.steps.contains(&"Create Quests"));
        assert!(app.workflow.steps.contains(&"Run Multiplayer"));
        assert!(app.workflow.steps.contains(&"Publish"));
        assert!(app.workflow.steps.contains(&"Players Join"));
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

    #[test]
    fn test_studio_multiplayer_workflow_equivalence() {
        let app = StudioGuiApp::new();
        for action in [
            "Create World",
            "Host World",
            "Join World",
            "Invite Players",
            "Share Link",
        ] {
            assert!(app.multiplayer.actions.contains(&action));
        }
        assert!(app.multiplayer.no_networking_setup);
        assert!(app
            .multiplayer
            .invite_link
            .starts_with("everarcade://join/"));
        assert!(app.workflow.steps.contains(&"World Live"));
        assert!(app.workflow.steps.contains(&"Players Join"));
        assert!(app.workflow.steps.contains(&"Operate World"));
    }

    #[test]
    fn test_world_operations_dashboard_equivalence() {
        let app = StudioGuiApp::new();
        for surface in [
            "online players",
            "world health",
            "simulation health",
            "runtime health",
            "replay health",
            "deployment health",
        ] {
            assert!(app.operations_dashboard.health_surfaces.contains(&surface));
        }
        for metric in [
            "entity counts",
            "simulation load",
            "partition load",
            "scheduler load",
            "runtime latency",
            "replay continuity",
        ] {
            assert!(app.operations_dashboard.metrics.contains(&metric));
        }
        assert!(app.operations_dashboard.single_dashboard);
    }

    #[test]
    fn test_creator_administration_layer_equivalence() {
        let app = StudioGuiApp::new();
        for control in [
            "world settings",
            "player management",
            "runtime controls",
            "deployment controls",
            "rollback controls",
        ] {
            assert!(app.operations_dashboard.admin_controls.contains(&control));
        }
        assert!(app.request_authority_mutation(true).is_err());
    }

    #[test]
    fn test_creator_pipeline_equivalence() {
        let app = StudioGuiApp::new();
        for step in [
            "Install Studio",
            "Create Project",
            "Choose Template",
            "Create Gameplay",
            "Build World",
            "Run Multiplayer",
            "Publish",
            "Players Join",
        ] {
            assert!(app.workflow.steps.contains(&step));
        }
        assert!(app.gui_readiness());
    }

    #[test]
    fn test_template_generation_equivalence() {
        let templates = world_authoring::template_catalog();
        assert_eq!(templates.len(), 8);
        assert!(templates.iter().all(|template| template.runnable));
        assert_eq!(templates, world_authoring::template_catalog());
    }

    #[test]
    fn test_play_mode_equivalence() {
        let app = StudioGuiApp::new();
        for mode in [
            "single player",
            "multiplayer",
            "persistent world",
            "replay mode",
        ] {
            assert!(app.workflow.play_modes.contains(&mode));
        }
    }

    #[test]
    fn test_multiplayer_creation_equivalence() {
        let app = StudioGuiApp::new();
        assert!(app.workflow.steps.contains(&"Run Multiplayer"));
        assert!(app.multiplayer.no_networking_setup);
        assert!(app.multiplayer.actions.contains(&"Host World"));
    }

    #[test]
    fn test_package_generation_equivalence() {
        let app = StudioGuiApp::new();
        for artifact in [
            "game package",
            "runtime package",
            "world package",
            "asset package",
            "deployment package",
        ] {
            assert!(app.workflow.package_artifacts.contains(&artifact));
        }
    }

    #[test]
    fn test_publish_workflow_equivalence() {
        let mut world = world_authoring::WorldAuthoringState::sample();
        assert_eq!(world.publish_game().unwrap(), "Game Live");
        assert!(world.publish.infrastructure_hidden);
    }

    #[test]
    fn test_protocol_readiness() {
        let app = StudioGuiApp::new();
        for report in [
            "protocol readiness",
            "creator readiness",
            "deployment readiness",
            "ecosystem readiness",
        ] {
            assert!(app.workflow.readiness_reports.contains(&report));
        }
    }

    #[test]
    fn test_creator_readiness() {
        let app = StudioGuiApp::new();
        assert!(app.gui_readiness());
        assert!(app.workflow.metrics_opt_in_local_only);
        assert!(app.workflow.single_validation_action);
    }

    #[test]
    fn test_replay_safe_pipeline() {
        let app = StudioGuiApp::new();
        assert!(app
            .workflow
            .validation_checks
            .contains(&"replay validation"));
        assert!(app.replay.request_replay_mutation(true).is_err());
        assert!(app.request_authority_mutation(true).is_err());
    }
}
