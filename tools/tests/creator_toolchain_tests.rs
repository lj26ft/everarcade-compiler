use tools::{
    asset_pipeline, assets, content_packager, creator_dashboard, deployment, diagnostics,
    ecs_editor, editor, entity_inspector, hierarchy, hot_reload, inspector, publishing, replay,
    replay_visualizer, simulation, simulation_debugger, studio, viewport, world_builder,
};

const FRAMES: &[&str] = &["tick=1,state=a", "tick=2,state=b", "tick=3,state=c"];
const COMPONENTS: &[&str] = &["ai", "position", "velocity"];

#[test]
fn test_editor_replay_equivalence() {
    assert!(editor::validation::editor_replay_equivalence(FRAMES));
    assert!(editor::runtime::request_authority_bypass(false).is_ok());
    assert!(editor::runtime::request_authority_bypass(true).is_err());
}

#[test]
fn test_ecs_authoring_equivalence() {
    assert!(ecs_editor::validation::ecs_authoring_equivalence(
        COMPONENTS
    ));
    assert!(ecs_editor::system::reject_nondeterministic_order(&[
        "ai",
        "physics",
        "render-projection"
    ])
    .is_ok());
    assert!(ecs_editor::system::reject_nondeterministic_order(&["physics", "ai"]).is_err());
}

#[test]
fn test_replay_visualizer_equivalence() {
    assert!(replay_visualizer::session::replay_visualizer_equivalence(
        FRAMES
    ));
    assert!(replay_visualizer::session::request_replay_mutation(true).is_err());
}

#[test]
fn test_asset_pipeline_hash_equivalence() {
    assert!(asset_pipeline::validation::asset_pipeline_hash_equivalence(
        "hero",
        b"pixel-data"
    ));
    assert!(asset_pipeline::validation::validate_asset_compatibility("image").is_ok());
    assert!(asset_pipeline::validation::validate_asset_compatibility("wall-clock-plugin").is_err());
}

#[test]
fn test_hot_reload_restoration() {
    assert!(hot_reload::validation::hot_reload_restoration(
        "state-root-1"
    ));
    assert!(hot_reload::validation::reject_invalid_reload(true).is_err());
}

#[test]
fn test_entity_inspector_replay_safety() {
    assert!(entity_inspector::validation::entity_inspector_replay_safety("entity-1", COMPONENTS));
    assert!(entity_inspector::validation::request_authority_bypass(true).is_err());
}

#[test]
fn test_simulation_debugger_equivalence() {
    assert!(simulation_debugger::validation::simulation_debugger_equivalence(FRAMES));
    assert!(simulation_debugger::validation::request_authority_mutation(true).is_err());
}

#[test]
fn test_content_package_hash_equivalence() {
    assert!(
        content_packager::validation::content_package_hash_equivalence(
            "arena",
            &["asset-a", "asset-b"]
        )
    );
    assert!(content_packager::runtime::validate_runtime_compatibility("everarcade-0.1").is_ok());
    assert!(content_packager::runtime::validate_runtime_compatibility("foreign-runtime").is_err());
}

#[test]
fn test_content_registry_continuity() {
    assert!(content_registry::validation::content_registry_continuity(
        "arena", "hash-a"
    ));
    assert!(content_registry::validation::reject_invalid_package_mutation(true).is_err());
}

#[test]
fn test_creator_dashboard_equivalence() {
    assert!(creator_dashboard::validation::creator_dashboard_equivalence("project-1"));
    assert!(creator_dashboard::validation::reject_invalid_package_mutation(true).is_err());
}

#[test]
fn test_authority_mutation_rejection() {
    assert!(tools::reject_authority_bypass(true).is_err());
    assert!(tools::reject_replay_mutation(true).is_err());
}

#[test]
fn test_replay_safe_creator_surfaces() {
    for diagnostic in [
        editor::validation::validate_editor_surface(),
        ecs_editor::validation::validate_ecs_authoring(),
        replay_visualizer::session::replay_visualizer_diagnostic(),
        hot_reload::validation::validate_hot_reload("checkpoint-a"),
        entity_inspector::validation::validate_entity_inspector(),
        simulation_debugger::validation::validate_simulation_debugger(),
        creator_dashboard::validation::validate_creator_dashboard(),
        content_packager::validation::validate_content_packaging(),
    ] {
        assert!(diagnostic.deterministic);
        assert_eq!(diagnostic.replay_continuity, "preserved");
        assert!(!diagnostic.renderer_authoritative);
        assert_eq!(
            diagnostic.authority_boundary,
            "deterministic-execution-runtime-only"
        );
    }
}

#[test]
fn test_world_builder_equivalence() {
    assert!(world_builder::validation::world_builder_equivalence());
    assert!(world_builder::placement::reject_hidden_state_mutation(true).is_err());
}

#[test]
fn test_asset_pipeline_equivalence() {
    assert!(assets::validation::asset_pipeline_equivalence());
}

#[test]
fn test_viewport_projection_integrity() {
    assert!(viewport::validation::viewport_projection_integrity());
}

#[test]
fn test_replay_timeline_equivalence() {
    assert!(replay::validation::replay_timeline_equivalence(FRAMES));
}

#[test]
fn test_simulation_visualizer_equivalence() {
    assert!(simulation::validation::simulation_visualizer_equivalence());
}

#[test]
fn test_package_hash_equivalence() {
    assert!(publishing::validation::package_hash_equivalence());
}

#[test]
fn test_deployment_continuity() {
    assert!(deployment::validation::deployment_continuity());
}

#[test]
fn test_creator_workflow_equivalence() {
    assert!(studio::validation::studio_launches());
    assert!(studio::validation::studio_workflow_equivalence("project-1"));
}

#[test]
fn test_replay_safe_studio_surfaces() {
    for diagnostic in [
        studio::validation::validate_studio_surface(),
        world_builder::validation::validate_world_builder(),
        viewport::validation::validate_viewport(),
        hierarchy::validation::validate_hierarchy(),
        inspector::validation::validate_inspector(),
        assets::validation::validate_asset_browser(),
        replay::validation::validate_replay_ui(),
        simulation::validation::validate_simulation_visualizer(),
        diagnostics::validation::validate_diagnostics(),
        publishing::validation::validate_publishing(),
        deployment::validation::validate_deployment_ux(),
    ] {
        assert!(diagnostic.deterministic);
        assert_eq!(diagnostic.replay_continuity, "preserved");
        assert!(!diagnostic.renderer_authoritative);
        assert_eq!(
            diagnostic.authority_boundary,
            "deterministic-execution-runtime-only"
        );
    }
    assert!(studio::runtime::request_authority_bypass(true).is_err());
    assert!(inspector::runtime::request_direct_mutation(true).is_err());
}

#[test]
fn test_world_authoring_equivalence() {
    assert!(tools::creator_productization::world_authoring_equivalence());
}

#[test]
fn test_gizmo_equivalence() {
    assert!(tools::creator_productization::gizmo_equivalence());
}

#[test]
fn test_scene_graph_equivalence() {
    assert!(tools::creator_productization::scene_graph_equivalence());
}

#[test]
fn test_live_edit_equivalence() {
    assert!(tools::creator_productization::live_edit_equivalence());
}

#[test]
fn test_simulation_control_equivalence() {
    assert!(tools::creator_productization::simulation_control_equivalence());
}

#[test]
fn test_visual_replay_timeline_equivalence() {
    assert!(tools::creator_productization::replay_timeline_equivalence());
}

#[test]
fn test_asset_dragdrop_equivalence() {
    assert!(tools::creator_productization::asset_dragdrop_equivalence());
}

#[test]
fn test_template_creation_equivalence() {
    assert!(tools::creator_productization::template_creation_equivalence());
}

#[test]
fn test_local_runtime_launch() {
    assert!(tools::creator_productization::local_runtime_launch());
}

#[test]
fn test_publish_pipeline_equivalence() {
    assert!(tools::creator_productization::publish_pipeline_equivalence());
}

#[test]
fn test_entity_placement_equivalence() {
    assert!(tools::creator_productization::entity_placement_equivalence());
}

#[test]
fn test_terrain_authoring_equivalence() {
    assert!(tools::creator_productization::terrain_authoring_equivalence());
}

#[test]
fn test_asset_import_equivalence() {
    assert!(tools::creator_productization::asset_import_equivalence());
}

#[test]
fn test_live_simulation_equivalence() {
    assert!(tools::creator_productization::live_simulation_equivalence());
}

#[test]
fn test_replay_visualization_equivalence() {
    assert!(tools::creator_productization::replay_visualization_equivalence());
}

#[test]
fn test_world_save_load_equivalence() {
    assert!(tools::creator_productization::world_save_load_equivalence());
}

#[test]
fn test_undo_redo_equivalence() {
    assert!(tools::creator_productization::undo_redo_equivalence());
}

#[test]
fn test_replay_safe_creator_workflow() {
    assert!(tools::creator_productization::replay_safe_creator_workflow());
}

#[test]
fn test_creator_pipeline_equivalence() {
    assert!(tools::creator_pipeline::creator_pipeline_equivalence());
}

#[test]
fn test_template_generation_equivalence() {
    assert!(tools::creator_pipeline::template_generation_equivalence());
}

#[test]
fn test_play_mode_equivalence() {
    assert!(tools::creator_pipeline::play_mode_equivalence());
}

#[test]
fn test_multiplayer_creation_equivalence() {
    assert!(tools::creator_pipeline::multiplayer_creation_equivalence());
}

#[test]
fn test_package_generation_equivalence() {
    assert!(tools::creator_pipeline::package_generation_equivalence());
}

#[test]
fn test_publish_workflow_equivalence() {
    assert!(tools::creator_pipeline::publish_workflow_equivalence());
}

#[test]
fn test_protocol_readiness() {
    assert!(tools::creator_pipeline::protocol_readiness_equivalence());
}

#[test]
fn test_creator_readiness() {
    assert!(tools::creator_pipeline::creator_readiness_equivalence());
}

#[test]
fn test_replay_safe_pipeline() {
    assert!(tools::creator_pipeline::replay_safe_pipeline());
}
