# Scripts Manifest

This manifest is a contributor map for `scripts/`. It documents current script names without renaming or moving them, preserving launch history while making the directory navigable.

General rules: run scripts from the repository root; prefer the targeted script for the area you changed; use `CARGO_BUILD_JOBS=1` for large validation runs; do not treat scaffold-domain PASS output as production readiness.

## Onboarding

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/bootstrap.sh` | Bootstrap helper for bootstrap. | Run during first-clone setup or when diagnosing local prerequisites. | No, not by default. | No; contributors may run when relevant. |
| `scripts/bootstrap_e2e.sh` | Bootstrap helper for e2e. | Run during first-clone setup or when diagnosing local prerequisites. | No, not by default. | No; contributors may run when relevant. |
| `scripts/install.sh` | Install helper for install. | Run during first-clone setup or when diagnosing local prerequisites. | No, not by default. | No; contributors may run when relevant. |
| `scripts/install_smoke.sh` | Install helper for smoke. | Run during first-clone setup or when diagnosing local prerequisites. | No, not by default. | No; contributors may run when relevant. |
| `scripts/test_fresh_bootstrap_paths.sh` | Test fresh bootstrap paths. | Run during first-clone setup or when diagnosing local prerequisites. | No, not by default. | No; contributors may run when relevant. |
| `scripts/uninstall.sh` | Uninstall. | Run during first-clone setup or when diagnosing local prerequisites. | No, not by default. | No; contributors may run when relevant. |

## CI

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/ci/check-rc2-commit-pins.sh` | Check rc2 commit pins. | Run when reproducing the matching CI gate locally. | Yes, directly or as a protected gate. | No; contributors may run when relevant. |
| `scripts/ci/rc2-fixture-self-attested-fork-must-fail.sh` | Rc2 fixture self attested fork must fail. | Run when reproducing the matching CI gate locally. | Yes, directly or as a protected gate. | No; contributors may run when relevant. |
| `scripts/ci/rc2-fixture-tampered-payload-must-fail.sh` | Rc2 fixture tampered payload must fail. | Run when reproducing the matching CI gate locally. | Yes, directly or as a protected gate. | No; contributors may run when relevant. |
| `scripts/ci/rc2-gate.sh` | Rc2 gate. | Run when reproducing the matching CI gate locally. | Yes, directly or as a protected gate. | No; contributors may run when relevant. |
| `scripts/ci/run-deterministic-world-factory.sh` | Run deterministic world factory. | Run when reproducing the matching CI gate locally. | Yes, directly or as a protected gate. | No; contributors may run when relevant. |

## Verification

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/certify_asset_ownership_continuity.sh` | Certification-style proof script for asset ownership continuity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_civilization_runtime.sh` | Certification-style proof script for civilization runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_clean_clone_runtime.sh` | Certification-style proof script for clean clone runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_commercial_revenue.sh` | Certification-style proof script for commercial revenue. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_continuity_anchor_proof.sh` | Certification-style proof script for continuity anchor proof. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_creator_marketplace.sh` | Certification-style proof script for creator marketplace. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_creator_sdk.sh` | Certification-style proof script for creator sdk. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_deterministic_execution.sh` | Certification-style proof script for deterministic execution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_deterministic_physics.sh` | Certification-style proof script for deterministic physics. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_developer_experience.sh` | Certification-style proof script for developer experience. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_developer_portal.sh` | Certification-style proof script for developer portal. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_economic_ledger.sh` | Certification-style proof script for economic ledger. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_federated_settlement.sh` | Certification-style proof script for federated settlement. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_fresh_vm_runtime.sh` | Certification-style proof script for fresh vm runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_game_discovery.sh` | Certification-style proof script for game discovery. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_game_templates.sh` | Certification-style proof script for game templates. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_governance_authority.sh` | Certification-style proof script for governance authority. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_gpu_marketplace.sh` | Certification-style proof script for gpu marketplace. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_gpu_runtime.sh` | Certification-style proof script for gpu runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_inventory_continuity.sh` | Certification-style proof script for inventory continuity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_local_game_launch.sh` | Certification-style proof script for local game launch. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_marketplace_transactions.sh` | Certification-style proof script for marketplace transactions. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_multiplayer_local_session.sh` | Certification-style proof script for multiplayer local session. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_network_transport_session.sh` | Certification-style proof script for network transport session. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_playable_local_game.sh` | Certification-style proof script for playable local game. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_player_gateway.sh` | Certification-style proof script for player gateway. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_protocol_sovereignty.sh` | Certification-style proof script for protocol sovereignty. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_renderer_runtime.sh` | Certification-style proof script for renderer runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_template_gameplay_execution.sh` | Certification-style proof script for template gameplay execution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_tenant_runtime.sh` | Certification-style proof script for tenant runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_vault_ownership.sh` | Certification-style proof script for vault ownership. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_wallet_authority.sh` | Certification-style proof script for wallet authority. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_wasm_guest_execution.sh` | Certification-style proof script for wasm guest execution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/check_no_empty_tests.sh` | Preflight/check script for no empty tests. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/check_prerequisites.sh` | Preflight/check script for prerequisites. | Run when your change touches the named subsystem or evidence path. | Yes, directly or as a protected gate. | No; contributors may run when relevant. |
| `scripts/compare-consensus-roots.sh` | Compare consensus roots. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/detect-divergence.sh` | Detect divergence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/detect-live-divergence.sh` | Detect live divergence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | Yes. |
| `scripts/run_anchor_validation.sh` | Validation script for anchor. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_arena_vanguard_playable_validation.sh` | Validation script for arena vanguard playable. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_arena_vanguard_rustrig_usage_validation.sh` | Validation script for arena vanguard rustrig usage. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_arena_vanguard_validation.sh` | Validation script for arena vanguard. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_asset_authoring_validation.sh` | Validation script for asset authoring. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_asset_pipeline_validation.sh` | Validation script for asset pipeline. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_authoritative_scheduler_validation.sh` | Validation script for authoritative scheduler. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_behavior_tree_validation.sh` | Validation script for behavior tree. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_browser_multiplayer_validation.sh` | Validation script for browser multiplayer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_browser_reconnect_validation.sh` | Validation script for browser reconnect. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_checkpoint_restore_validation.sh` | Validation script for checkpoint restore. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_ci_execution_replay_validation.sh` | Validation script for ci execution replay. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_ci_pipeline_validation.sh` | Validation script for ci pipeline. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_ci_scheduler_validation.sh` | Validation script for ci scheduler. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_civilization_archive_validation.sh` | Validation script for civilization archive. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_civilization_federation_validation.sh` | Validation script for civilization federation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_civilization_scheduler_validation.sh` | Validation script for civilization scheduler. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_civilization_validation.sh` | Validation script for civilization. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_component_editor_validation.sh` | Validation script for component editor. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_content_packaging_validation.sh` | Validation script for content packaging. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_content_registry_validation.sh` | Validation script for content registry. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_convergence_validation.sh` | Validation script for convergence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_cost_model_validation.sh` | Validation script for cost model. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_creator_dashboard_validation.sh` | Validation script for creator dashboard. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_creator_pipeline_validation.sh` | Validation script for creator pipeline. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_creator_production_validation.sh` | Validation script for creator production. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_creator_workflow_validation.sh` | Validation script for creator workflow. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_cross_machine_certification.sh` | Runnable proof or workflow for cross machine certification. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_cross_machine_long_duration_validation.sh` | Validation script for cross machine long duration. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_cross_machine_partition_validation.sh` | Validation script for cross machine partition. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_cross_machine_transport_validation.sh` | Validation script for cross machine transport. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_determinism_audit.sh` | Runnable proof or workflow for determinism audit. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_distributed_transport_validation.sh` | Validation script for distributed transport. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_distribution_validation.sh` | Validation script for distribution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_ecology_validation.sh` | Validation script for ecology. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_economy_validation.sh` | Validation script for economy. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_ecs_editor_validation.sh` | Validation script for ecs editor. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_ecs_validation.sh` | Validation script for ecs. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_editor_validation.sh` | Validation script for editor. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_encrypted_transport_validation.sh` | Validation script for encrypted transport. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_entity_inspector_validation.sh` | Validation script for entity inspector. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_epoch_materialization_validation.sh` | Validation script for epoch materialization. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_epoch_validation.sh` | Validation script for epoch. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_executable_rustrigs_validation.sh` | Validation script for executable rustrigs. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_execution_integrity_validation.sh` | Validation script for execution integrity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_execution_replay_validation.sh` | Validation script for execution replay. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_federation_certification.sh` | Runnable proof or workflow for federation certification. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_federation_operations_validation.sh` | Validation script for federation operations. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_federation_simulation.sh` | Runnable proof or workflow for federation simulation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_federation_validation.sh` | Validation script for federation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_frontend_readiness_validation.sh` | Validation script for frontend readiness. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_frontend_validation.sh` | Validation script for frontend. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_fuel_metering_validation.sh` | Validation script for fuel metering. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_gateway_binding_validation.sh` | Validation script for gateway binding. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_global_observer_validation.sh` | Validation script for global observer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_governance_validation.sh` | Validation script for governance. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_graphical_renderer_validation.sh` | Validation script for graphical renderer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_historical_archive_roundtrip_validation.sh` | Validation script for historical archive roundtrip. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_historical_cache_validation.sh` | Validation script for historical cache. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_historical_dataplane_validation.sh` | Validation script for historical dataplane. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_historical_index_validation.sh` | Validation script for historical index. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_historical_observer_validation.sh` | Validation script for historical observer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_historical_replay_validation.sh` | Validation script for historical replay. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_historical_restoration_validation.sh` | Validation script for historical restoration. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_hot_reload_validation.sh` | Validation script for hot reload. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_incremental_replay_validation.sh` | Validation script for incremental replay. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_interactive_editing_validation.sh` | Validation script for interactive editing. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_inventory_validation.sh` | Validation script for inventory. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_json_contract_validation.sh` | Validation script for json contract. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_launch_candidate_validation.sh` | Validation script for launch candidate. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_launch_readiness_validation.sh` | Validation script for launch readiness. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_live_hosted_validation.sh` | Validation script for live hosted. | Run when your change touches the named subsystem or evidence path. | No, not by default. | Yes. |
| `scripts/run_live_peer_io_validation.sh` | Validation script for live peer io. | Run when your change touches the named subsystem or evidence path. | No, not by default. | Yes. |
| `scripts/run_live_session_validation.sh` | Validation script for live session. | Run when your change touches the named subsystem or evidence path. | No, not by default. | Yes. |
| `scripts/run_live_simulation_validation.sh` | Validation script for live simulation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | Yes. |
| `scripts/run_low_resource_workspace_validation.sh` | Validation script for low resource workspace. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_manifest_validation.sh` | Validation script for manifest. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_matchmaking_validation.sh` | Validation script for matchmaking. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_multiplayer_sdk_validation.sh` | Validation script for multiplayer sdk. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_multiplayer_session_validation.sh` | Validation script for multiplayer session. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_multiplayer_validation.sh` | Validation script for multiplayer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_observability_validation.sh` | Validation script for observability. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_observer_civilization_validation.sh` | Validation script for observer civilization. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_observer_gameplay_validation.sh` | Validation script for observer gameplay. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_observer_simulation_validation.sh` | Validation script for observer simulation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_operational_audit.sh` | Runnable proof or workflow for operational audit. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_operational_governance_validation.sh` | Validation script for operational governance. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_operational_store_validation.sh` | Validation script for operational store. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_packaging_validation.sh` | Validation script for packaging. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_partition_execution_validation.sh` | Validation script for partition execution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_partition_validation.sh` | Validation script for partition. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_partitioned_workspace_validation.sh` | Validation script for partitioned workspace. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_player_connectivity_validation.sh` | Validation script for player connectivity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_player_portal_validation.sh` | Validation script for player portal. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_player_session_validation.sh` | Validation script for player session. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_playmode_validation.sh` | Validation script for playmode. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_product_command_validation.sh` | Validation script for product command. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_adversarial_validation.sh` | Validation script for projection adversarial. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_archive_distribution_validation.sh` | Validation script for projection archive distribution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_archive_validation.sh` | Validation script for projection archive. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_cryptographic_validation.sh` | Validation script for projection cryptographic. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_federation_validation.sh` | Validation script for projection federation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_observer_validation.sh` | Validation script for projection observer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_replay_validation.sh` | Validation script for projection replay. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_stream_validation.sh` | Validation script for projection stream. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_protocol_readiness_validation.sh` | Validation script for protocol readiness. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_protocol_record_validation.sh` | Validation script for protocol record. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_provider_observability_validation.sh` | Validation script for provider observability. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_publish_pipeline_validation.sh` | Validation script for publish pipeline. | Run when your change touches the named subsystem or evidence path. | No, not by default. | Yes. |
| `scripts/run_quic_transport_validation.sh` | Validation script for quic transport. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_registry_validation.sh` | Validation script for registry. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_anchor_validation.sh` | Validation script for replay anchor. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_archive_hydration_validation.sh` | Validation script for replay archive hydration. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_branch_validation.sh` | Validation script for replay branch. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_catchup_validation.sh` | Validation script for replay catchup. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_compression_validation.sh` | Validation script for replay compression. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_debugger_validation.sh` | Validation script for replay debugger. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_equivalence_validation.sh` | Validation script for replay equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_federation_activation_validation.sh` | Validation script for replay federation activation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_federation_validation.sh` | Validation script for replay federation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_network_operational_validation.sh` | Validation script for replay network operational. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_network_validation.sh` | Validation script for replay network. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_observer_validation.sh` | Validation script for replay observer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_peer_validation.sh` | Validation script for replay peer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_persistence_validation.sh` | Validation script for replay persistence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_proof_validation.sh` | Validation script for replay proof. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_provenance_validation.sh` | Validation script for replay provenance. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_shard_validation.sh` | Validation script for replay shard. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_transport_activation_validation.sh` | Validation script for replay transport activation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_transport_integrity.sh` | Runnable proof or workflow for replay transport integrity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_transport_runtime.sh` | Runnable proof or workflow for replay transport runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_ui_validation.sh` | Validation script for replay ui. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_visualizer_validation.sh` | Validation script for replay visualizer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_window_validation.sh` | Validation script for replay window. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_restoration_validation.sh` | Validation script for restoration. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_restore_validation.sh` | Validation script for restore. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_rustrig_marketplace_validation.sh` | Validation script for rustrig marketplace. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_rustrig_validation.sh` | Validation script for rustrig. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_scheduler_validation.sh` | Validation script for scheduler. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_sdk_validation.sh` | Validation script for sdk. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_security_validation.sh` | Validation script for security. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_shard_migration_validation.sh` | Validation script for shard migration. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_simulation_debugger_validation.sh` | Validation script for simulation debugger. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_simulation_federation_validation.sh` | Validation script for simulation federation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_simulation_scheduler_validation.sh` | Validation script for simulation scheduler. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_snapshot_restoration_validation.sh` | Validation script for snapshot restoration. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_social_memory_validation.sh` | Validation script for social memory. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_stateful_execution_validation.sh` | Validation script for stateful execution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_storage_fabric_validation.sh` | Validation script for storage fabric. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_studio_gui_validation.sh` | Validation script for studio gui. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_tcp_replay_transport_validation.sh` | Validation script for tcp replay transport. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_template_validation.sh` | Validation script for template. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_terrain_authoring_validation.sh` | Validation script for terrain authoring. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_trade_conflict_validation.sh` | Validation script for trade conflict. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_validation_cleanup.sh` | Validation script for validation cleanup. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_validation_dag.sh` | Validation script for validation dag. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_validation_disk_preflight.sh` | Validation script for validation disk preflight. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_validation_pressure_profile.sh` | Validation script for validation pressure profile. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_viewport_validation.sh` | Validation script for viewport. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_visual_editing_validation.sh` | Validation script for visual editing. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_warning_cleanup_validation.sh` | Validation script for warning cleanup. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_websocket_observer_validation.sh` | Validation script for websocket observer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_websocket_streaming_validation.sh` | Validation script for websocket streaming. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_witness_validation.sh` | Validation script for witness. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_workspace_partition_validation.sh` | Validation script for workspace partition. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_workspace_warning_gate.sh` | Runnable proof or workflow for workspace warning gate. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate.sh` | Validate. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_archive_integrity.sh` | Validation script for archive integrity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_archive_sync.sh` | Validation script for archive sync. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_civilization_archive.sh` | Validation script for civilization archive. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_clean_vm_bootstrap.sh` | Validation script for clean vm bootstrap. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_commercial_revenue.sh` | Validation script for commercial revenue. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_creator_marketplace.sh` | Validation script for creator marketplace. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_creator_sdk.sh` | Validation script for creator sdk. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_deterministic_execution.sh` | Validation script for deterministic execution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_deterministic_execution_runtime.sh` | Validation script for deterministic execution runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_deterministic_federation_runtime.sh` | Validation script for deterministic federation runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_developer_onboarding.sh` | Validation script for developer onboarding. | Run when your change touches the named subsystem or evidence path. | Yes, directly or as a protected gate. | No; contributors may run when relevant. |
| `scripts/validate_developer_portal.sh` | Validation script for developer portal. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_federation_equivalence.sh` | Validation script for federation equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_federation_quarantine.sh` | Validation script for federation quarantine. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_frontend_access.sh` | Validation script for frontend access. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_game_discovery.sh` | Validation script for game discovery. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_game_templates.sh` | Validation script for game templates. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_gpu_marketplace.sh` | Validation script for gpu marketplace. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_gpu_runtime.sh` | Validation script for gpu runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_local_game_launch.sh` | Validation script for local game launch. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_multiplayer_local_session.sh` | Validation script for multiplayer local session. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_network_transport_session.sh` | Validation script for network transport session. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_open_source_readiness.sh` | Validation script for open source readiness. | Run when your change touches the named subsystem or evidence path. | Yes, directly or as a protected gate. | No; contributors may run when relevant. |
| `scripts/validate_playable_local_game.sh` | Validation script for playable local game. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_player_gateway.sh` | Validation script for player gateway. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_renderer_runtime.sh` | Validation script for renderer runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_replay_corruption.sh` | Validation script for replay corruption. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_replay_sync.sh` | Validation script for replay sync. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_repo_reality_audit.sh` | Validation script for repo reality audit. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_runtime.sh` | Validation script for runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_scheduler_equivalence.sh` | Validation script for scheduler equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_scheduler_faults.sh` | Validation script for scheduler faults. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_sovereign_operations_layer.sh` | Validation script for sovereign operations layer. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_sovereign_protocol_coordination.sh` | Validation script for sovereign protocol coordination. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_template_gameplay_execution.sh` | Validation script for template gameplay execution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_wasm_guest_execution.sh` | Validation script for wasm guest execution. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_wasm_isolation.sh` | Validation script for wasm isolation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_wasm_protocol_runtime.sh` | Validation script for wasm protocol runtime. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_capability_negotiation.sh` | Verification script for capability negotiation. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_economic_ledger_equivalence.sh` | Verification script for economic ledger equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_epoch_transition_equivalence.sh` | Verification script for epoch transition equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_execution_equivalence.sh` | Verification script for execution equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_execution_replay.sh` | Verification script for execution replay. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_federation_checkpoint_continuity.sh` | Verification script for federation checkpoint continuity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_federation_replay_equivalence.sh` | Verification script for federation replay equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_federation_snapshot_equivalence.sh` | Verification script for federation snapshot equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_inventory_ownership_continuity.sh` | Verification script for inventory ownership continuity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_migration_replay_continuity.sh` | Verification script for migration replay continuity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_protocol_upgrade_continuity.sh` | Verification script for protocol upgrade continuity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_replay_compression_continuity.sh` | Verification script for replay compression continuity. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_replay_determinism.sh` | Verification script for replay determinism. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_topology_scaling_equivalence.sh` | Verification script for topology scaling equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_wasm_dag_equivalence.sh` | Verification script for wasm dag equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_wasm_execution_equivalence.sh` | Verification script for wasm execution equivalence. | Run when your change touches the named subsystem or evidence path. | No, not by default. | No; contributors may run when relevant. |

## World Factory

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/certify_persistent_world.sh` | Certification-style proof script for persistent world. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_bundle_generation.sh` | Runnable proof or workflow for bundle generation. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_observer_world_validation.sh` | Validation script for observer world. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_persistent_world_validation.sh` | Validation script for persistent world. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_procedural_world_validation.sh` | Validation script for procedural world. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_world_authoring_validation.sh` | Validation script for world authoring. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_world_operations_validation.sh` | Validation script for world operations. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_world_recovery_validation.sh` | Validation script for world recovery. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_world_restoration_validation.sh` | Validation script for world restoration. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_world_runtime_validation.sh` | Validation script for world runtime. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_world_simulation.sh` | Runnable proof or workflow for world simulation. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_world_simulation_validation.sh` | Validation script for world simulation. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_sovereign_world_persistence.sh` | Validation script for sovereign world persistence. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_world_continuity.sh` | Validation script for world continuity. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_world_restoration.sh` | Validation script for world restoration. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_world_archive_restoration.sh` | Verification script for world archive restoration. | Run when changing world authoring, packaging, fixtures, or `world.evr` evidence. | No, not by default. | No; contributors may run when relevant. |

## Release

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/build_evernode_package.sh` | Build/package helper for evernode package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/build_evernode_packages.sh` | Build/package helper for evernode packages. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/build_lease_handoff_package.sh` | Build/package helper for lease handoff package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/build_live_gameplay_proof_package.sh` | Build/package helper for live gameplay proof package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/build_release_artifact_bundle.sh` | Build/package helper for release artifact bundle. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/build_release_tarball.sh` | Build/package helper for release tarball. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/build_runtime_release.sh` | Build/package helper for runtime release. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/certify_deterministic_package_execution.sh` | Certification-style proof script for deterministic package execution. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/certify_lease_handoff_package.sh` | Certification-style proof script for lease handoff package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/certify_multi_package_isolation.sh` | Certification-style proof script for multi package isolation. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/certify_release_candidate.sh` | Certification-style proof script for release candidate. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/certify_stateful_package_persistence.sh` | Certification-style proof script for stateful package persistence. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/certify_vendor_artifact.sh` | Certification-style proof script for vendor artifact. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/check_no_generated_artifacts_tracked.sh` | Preflight/check script for no generated artifacts tracked. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/clean_generated_artifacts.sh` | Clean generated artifacts. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/ensure_vendor_offline.sh` | Ensure vendor offline. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/generate_evernode_packages.sh` | Generate evernode packages. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/hash_runtime_artifacts.sh` | Hash runtime artifacts. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/install_evernode_package.sh` | Install helper for evernode package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/install_lease_handoff_package.sh` | Install helper for lease handoff package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/package-formal-proof-target.js` | Package formal proof target. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/package_vendor_artifact.sh` | Package vendor artifact. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/preflight_vendor.sh` | Preflight vendor. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/release_package.sh` | Release package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/release_smoke.sh` | Release smoke. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/release_validate.sh` | Release validate. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/release_validate_fresh_vm.sh` | Release validate fresh vm. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/restore_vendor_artifact.sh` | Restore vendor artifact. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_artifact_retention_validation.sh` | Validation script for artifact retention. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_hotpocket_package_inspection.sh` | Runnable proof or workflow for hotpocket package inspection. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_hotpocket_package_proof.sh` | Runnable proof or workflow for hotpocket package proof. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_package_certification_validation.sh` | Validation script for package certification. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_projection_artifact_validation.sh` | Validation script for projection artifact. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_automation_validation.sh` | Validation script for release automation. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_candidate_validation.sh` | Validation script for release candidate. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_certification_validation.sh` | Validation script for release certification. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_gate_validation.sh` | Validation script for release gate. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_integrity_validation.sh` | Validation script for release integrity. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_lineage_validation.sh` | Validation script for release lineage. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_pipeline.sh` | Runnable proof or workflow for release pipeline. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_reproducibility_validation.sh` | Validation script for release reproducibility. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_signature_validation.sh` | Validation script for release signature. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_release_validation.sh` | Validation script for release. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/run_runtime_package_validation.sh` | Validation script for runtime package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/sign_runtime_manifest.sh` | Sign runtime manifest. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/validate_lease_handoff_package.sh` | Validation script for lease handoff package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/validate_release_candidate.sh` | Validation script for release candidate. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/vendor_deps.sh` | Vendor deps. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/verify_lease_handoff_package.sh` | Verification script for lease handoff package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/verify_release.sh` | Verification script for release. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/verify_release_artifact_bundle.sh` | Verification script for release artifact bundle. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/verify_runtime_package.sh` | Verification script for runtime package. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/verify_vendor_integrity.sh` | Verification script for vendor integrity. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |
| `scripts/verify_vendor_tree_hash.sh` | Verification script for vendor tree hash. | Run only while preparing or auditing release artifacts. | No, not by default. | Yes. |

## Deployment

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/build_runtime_appliance.sh` | Build/package helper for runtime appliance. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/build_vm_runtime_appliance.sh` | Build/package helper for vm runtime appliance. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_evernode_deployment.sh` | Certification-style proof script for evernode deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_evernode_live_gameplay.sh` | Certification-style proof script for evernode live gameplay. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_evernode_operations.sh` | Certification-style proof script for evernode operations. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_evernode_runtime.sh` | Certification-style proof script for evernode runtime. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_hotpocket_adapter.sh` | Certification-style proof script for hotpocket adapter. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_hotpocket_deployment.sh` | Certification-style proof script for hotpocket deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_hotpocket_execution.sh` | Certification-style proof script for hotpocket execution. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_hotpocket_gameplay.sh` | Certification-style proof script for hotpocket gameplay. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_hotpocket_integration.sh` | Certification-style proof script for hotpocket integration. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_hotpocket_migration.sh` | Certification-style proof script for hotpocket migration. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_hotpocket_runtime_integration.sh` | Certification-style proof script for hotpocket runtime integration. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_live_lease.sh` | Certification-style proof script for live lease. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_multi_lease_civilization.sh` | Certification-style proof script for multi lease civilization. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_multi_lease_federation.sh` | Certification-style proof script for multi lease federation. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_public_testnet.sh` | Certification-style proof script for public testnet. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_runtime_appliance.sh` | Certification-style proof script for runtime appliance. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_xaman_signing.sh` | Certification-style proof script for xaman signing. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_xrpl_anchor_publication.sh` | Certification-style proof script for xrpl anchor publication. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_xrpl_authority_mapping.sh` | Certification-style proof script for xrpl authority mapping. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_xrpl_live_settlement.sh` | Certification-style proof script for xrpl live settlement. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/certify_xrpl_settlement.sh` | Certification-style proof script for xrpl settlement. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/compare-live-hotpocket-roots.sh` | Compare live hotpocket roots. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/deploy_proof.sh` | Deploy proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/deploy_to_evernode_lease.sh` | Deploy to evernode lease. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/detect-live-hotpocket-divergence.sh` | Detect live hotpocket divergence. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/evernode_bootstrap.sh` | Evernode bootstrap. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/evernode_health.sh` | Evernode health. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/evernode_recover.sh` | Evernode recover. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/evernode_start.sh` | Evernode start. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/evernode_stop.sh` | Evernode stop. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/evernode_upgrade.sh` | Evernode upgrade. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/install_runtime.sh` | Install helper for runtime. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/install_runtime_appliance.sh` | Install helper for runtime appliance. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/lib_hotpocket_deployment_proof.sh` | Shared helper library for other scripts; do not run directly unless debugging script internals. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/lib_hotpocket_execution.sh` | Shared helper library for other scripts; do not run directly unless debugging script internals. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/local_xrpl_testnet.sh` | Local xrpl testnet. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/probe_evernode_environment.sh` | Probe evernode environment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/prove_evernode_lease_execution.sh` | Prove evernode lease execution. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_deployment_automation_validation.sh` | Validation script for deployment automation. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_deployment_validation.sh` | Validation script for deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_evernode_deployment_gate.sh` | Runnable proof or workflow for evernode deployment gate. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_evernode_deployment_validation.sh` | Validation script for evernode deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_evernode_provider_validation.sh` | Validation script for evernode provider. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_cluster_discovery_proof.sh` | Runnable proof or workflow for hotpocket cluster discovery proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_cluster_proposal_proof.sh` | Runnable proof or workflow for hotpocket cluster proposal proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_consensus_proof.sh` | Runnable proof or workflow for hotpocket consensus proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_contract_rehearsal.sh` | Runnable proof or workflow for hotpocket contract rehearsal. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_dependency_proof.sh` | Runnable proof or workflow for hotpocket dependency proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_deployment_proof.sh` | Runnable proof or workflow for hotpocket deployment proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_executable_resolution.sh` | Runnable proof or workflow for hotpocket executable resolution. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_execution_proof.sh` | Runnable proof or workflow for hotpocket execution proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_integration_validation.sh` | Validation script for hotpocket integration. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_journal_proof.sh` | Runnable proof or workflow for hotpocket journal proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_launch_proof.sh` | Runnable proof or workflow for hotpocket launch proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_packaging_validation.sh` | Validation script for hotpocket packaging. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_ping_proof.sh` | Runnable proof or workflow for hotpocket ping proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_receipt_proof.sh` | Runnable proof or workflow for hotpocket receipt proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_replay_proof.sh` | Runnable proof or workflow for hotpocket replay proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_hotpocket_roundtrip_proof.sh` | Runnable proof or workflow for hotpocket roundtrip proof. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_lease_manager_validation.sh` | Validation script for lease manager. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_live_deployment_validation.sh` | Validation script for live deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_runtime_deployment_orchestration.sh` | Runnable proof or workflow for runtime deployment orchestration. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_runtime_deployment_recovery.sh` | Runnable proof or workflow for runtime deployment recovery. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_runtime_deployment_validation.sh` | Validation script for runtime deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/run_xrpl_anchor_validation.sh` | Validation script for xrpl anchor. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_docs_deployment.sh` | Validation script for docs deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_evernode_deployment.sh` | Validation script for evernode deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_hotpocket_adapter.sh` | Validation script for hotpocket adapter. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_hotpocket_deployment.sh` | Validation script for hotpocket deployment. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_hotpocket_execution.sh` | Validation script for hotpocket execution. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_hotpocket_integration.sh` | Validation script for hotpocket integration. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_hotpocket_live_cluster.sh` | Validation script for hotpocket live cluster. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_hotpocket_runtime_boundary.sh` | Validation script for hotpocket runtime boundary. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_live_lease.sh` | Validation script for live lease. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_multi_lease_civilization.sh` | Validation script for multi lease civilization. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_multi_lease_federation.sh` | Validation script for multi lease federation. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_public_testnet.sh` | Validation script for public testnet. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_runtime_appliance.sh` | Validation script for runtime appliance. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_xaman_signing.sh` | Validation script for xaman signing. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/validate_xrpl_live_settlement.sh` | Validation script for xrpl live settlement. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |
| `scripts/verify_runtime_appliance_orchestration.sh` | Verification script for runtime appliance orchestration. | Run only for deployment proofs, local deployment rehearsals, or operator experiments. | No, not by default. | Yes. |

## Operator

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/bootstrap_runtime_from_bundle.sh` | Bootstrap helper for runtime from bundle. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/build_runtime_layout.sh` | Build/package helper for runtime layout. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_federated_runtime_sync.sh` | Certification-style proof script for federated runtime sync. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_protocol_node_appliance.sh` | Certification-style proof script for protocol node appliance. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_protocol_node_readiness.sh` | Certification-style proof script for protocol node readiness. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/certify_runtime_bootstrap.sh` | Certification-style proof script for runtime bootstrap. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/check_runtime_offline_gate.sh` | Preflight/check script for runtime offline gate. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/distributed_execution_recovery.sh` | Distributed execution recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/doctor_quick.sh` | Doctor quick. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/everarcade_start.sh` | Everarcade start. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/export-diagnostics.sh` | Export diagnostics. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/integration-continuity-status` | Integration continuity status. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/linux_vm_recovery.sh` | Linux vm recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/local_cluster.sh` | Local cluster. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/local_cluster_recovery.sh` | Local cluster recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/monitor_live_session_resources.sh` | Monitor live session resources. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | Yes. |
| `scripts/networked_cluster.sh` | Networked cluster. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_checkpoint.sh` | Node checkpoint. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_common.sh` | Shared helper library for other scripts; do not run directly unless debugging script internals. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_doctor.sh` | Node doctor. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_init.sh` | Node init. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_replay.sh` | Node replay. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_restore.sh` | Node restore. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_start.sh` | Node start. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_status.sh` | Node status. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/node_stop.sh` | Node stop. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/operator_failover.sh` | Operator failover. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/profile_runtime_tests.sh` | Profile runtime tests. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/reassignment_recovery.sh` | Reassignment recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | Yes. |
| `scripts/run_ai_runtime_validation.sh` | Validation script for ai runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_ci_recovery_validation.sh` | Validation script for ci recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_cross_machine_recovery.sh` | Runnable proof or workflow for cross machine recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_demo_runtime_validation.sh` | Validation script for demo runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_entity_runtime_validation.sh` | Validation script for entity runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_faction_runtime_validation.sh` | Validation script for faction runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_failure_recovery_validation.sh` | Validation script for failure recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_full_runtime_validation.sh` | Validation script for full runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_gameplay_recovery_validation.sh` | Validation script for gameplay recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_gameplay_runtime_validation.sh` | Validation script for gameplay runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_interactive_runtime_validation.sh` | Validation script for interactive runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_internet_runtime_validation.sh` | Validation script for internet runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_local_multinode_validation.sh` | Validation script for local multinode. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_multinode_federation_load_gate.sh` | Runnable proof or workflow for multinode federation load gate. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_multinode_federation_validation.sh` | Validation script for multinode federation. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_multiplayer_runtime_validation.sh` | Validation script for multiplayer runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_operator_console_validation.sh` | Validation script for operator console. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_operator_control_plane_validation.sh` | Validation script for operator control plane. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_operator_validation.sh` | Validation script for operator. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_recovery_validation.sh` | Validation script for projection recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_projection_runtime_integration_validation.sh` | Validation script for projection runtime integration. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_provider_recovery_validation.sh` | Validation script for provider recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_observer_runtime_validation.sh` | Validation script for replay observer runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_recovery_runtime_validation.sh` | Validation script for replay recovery runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_recovery_validation.sh` | Validation script for replay recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_runtime_activation.sh` | Runnable proof or workflow for replay runtime activation. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_replay_transport_runtime_validation.sh` | Validation script for replay transport runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_activation.sh` | Runnable proof or workflow for runtime activation. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_archive_exchange.sh` | Runnable proof or workflow for runtime archive exchange. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_archive_exchange_validation.sh` | Validation script for runtime archive exchange. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_autoscaling_validation.sh` | Validation script for runtime autoscaling. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_backpressure_validation.sh` | Validation script for runtime backpressure. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_bootstrap.sh` | Runnable proof or workflow for runtime bootstrap. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_connectivity_validation.sh` | Validation script for runtime connectivity. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_daemon_validation.sh` | Validation script for runtime daemon. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_distributed_transport.sh` | Runnable proof or workflow for runtime distributed transport. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_exhaustion_validation.sh` | Validation script for runtime exhaustion. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_federation_recovery.sh` | Runnable proof or workflow for runtime federation recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_federation_validation.sh` | Validation script for runtime federation. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_health_validation.sh` | Validation script for runtime health. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_isolation_validation.sh` | Validation script for runtime isolation. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_live_networking.sh` | Runnable proof or workflow for runtime live networking. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | Yes. |
| `scripts/run_runtime_node_lifecycle.sh` | Runnable proof or workflow for runtime node lifecycle. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_node_validation.sh` | Validation script for runtime node. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_observer_runtime.sh` | Runnable proof or workflow for runtime observer runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_observer_scaling.sh` | Runnable proof or workflow for runtime observer scaling. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_observer_stream_validation.sh` | Validation script for runtime observer stream. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_observer_validation.sh` | Validation script for runtime observer. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_operational_networking.sh` | Runnable proof or workflow for runtime operational networking. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_operations.sh` | Runnable proof or workflow for runtime operations. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_orchestration_validation.sh` | Validation script for runtime orchestration. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_peer_auth_validation.sh` | Validation script for runtime peer auth. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_peer_networking.sh` | Runnable proof or workflow for runtime peer networking. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_peer_session_validation.sh` | Validation script for runtime peer session. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_peer_topology.sh` | Runnable proof or workflow for runtime peer topology. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_peer_transport.sh` | Runnable proof or workflow for runtime peer transport. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_peer_validation.sh` | Validation script for runtime peer. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_quic_validation.sh` | Validation script for runtime quic. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_recovery.sh` | Runnable proof or workflow for runtime recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_recovery_runtime.sh` | Runnable proof or workflow for runtime recovery runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_recovery_validation.sh` | Validation script for runtime recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_replay_validation.sh` | Validation script for runtime replay. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_routing_validation.sh` | Validation script for runtime routing. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_security_validation.sh` | Validation script for runtime security. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_service_recovery.sh` | Runnable proof or workflow for runtime service recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_service_validation.sh` | Validation script for runtime service. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_session_validation.sh` | Validation script for runtime session. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_storage_validation.sh` | Validation script for runtime storage. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_stream_validation.sh` | Validation script for runtime stream. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_streaming_validation.sh` | Validation script for runtime streaming. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_stress_validation.sh` | Validation script for runtime stress. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_surface_audit.sh` | Runnable proof or workflow for runtime surface audit. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_topology_validation.sh` | Validation script for runtime topology. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_transport_boundary.sh` | Runnable proof or workflow for runtime transport boundary. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_runtime_validation_pipeline.sh` | Validation script for runtime pipeline. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_rustrig_runtime_validation.sh` | Validation script for rustrig runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_session_recovery_validation.sh` | Validation script for session recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_society_runtime_validation.sh` | Validation script for society runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_stdout_runtime_validation.sh` | Validation script for stdout runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_two_node_certification.sh` | Runnable proof or workflow for two node certification. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_validation_recovery.sh` | Validation script for validation recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_visible_runtime_validation.sh` | Validation script for visible runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_wan_recovery_validation.sh` | Validation script for wan recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/run_wasm_runtime_validation.sh` | Validation script for wasm runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/runtime-export-status` | Runtime export status. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/runtime_doctor.sh` | Runtime doctor. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/runtime_start.sh` | Runtime start. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/runtime_status.sh` | Runtime status. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/runtime_stop.sh` | Runtime stop. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/shutdown.sh` | Shutdown. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/stale_node_recovery.sh` | Stale node recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/start.sh` | Start. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/start_live_arena_session.sh` | Start live arena session. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | Yes. |
| `scripts/start_runtime.sh` | Start runtime. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_crash_recovery.sh` | Validation script for crash recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_federated_runtime_sync.sh` | Validation script for federated runtime sync. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_federation_recovery.sh` | Validation script for federation recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_live_session_recovery.sh` | Validation script for live session recovery. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | Yes. |
| `scripts/validate_runtime_distribution.sh` | Validation script for runtime distribution. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/validate_runtime_platform.sh` | Validation script for runtime platform. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_federation_recovery_operations.sh` | Verification script for federation recovery operations. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_replay_diagnostics_equivalence.sh` | Verification script for replay diagnostics equivalence. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | No; contributors may run when relevant. |
| `scripts/verify_runtime_manifest_signature.sh` | Verification script for runtime manifest signature. | Run when operating or diagnosing local runtime/node processes. | No, not by default. | Yes. |

## Development

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/cross_machine_repro_check.sh` | Cross machine repro check. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/distributed_execution.sh` | Distributed execution. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/distributed_receipt_sync.sh` | Distributed receipt sync. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/generate_performance_report.sh` | Generate performance report. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/lib/arena_live_runtime.mjs` | Shared helper library for other scripts; do not run directly unless debugging script internals. | Run during focused development of the named subsystem. | No, not by default. | Yes. |
| `scripts/lib/common.sh` | Shared helper library for other scripts; do not run directly unless debugging script internals. | Run during focused development of the named subsystem. | No, not by default. | Usually; library script. |
| `scripts/lint_v0_1_architecture_freeze.py` | Lint v0 1 architecture freeze. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/linux_vm_smoke.sh` | Linux vm smoke. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/linux_vm_stress.sh` | Linux vm stress. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/local_ipfs_publish.sh` | Local ipfs publish. | Run during focused development of the named subsystem. | No, not by default. | Yes. |
| `scripts/network_partition.sh` | Network partition. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/networked_checkpoint_sync.sh` | Networked checkpoint sync. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/networked_receipt_propagation.sh` | Networked receipt propagation. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/probe_network_failure_behavior.sh` | Probe network failure behavior. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/repo_size_audit.sh` | Repo size audit. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/resumable_sync.sh` | Resumable sync. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/runtime-dependency-graph` | Runtime dependency graph. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/runtime-namespace-audit` | Runtime namespace audit. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/test_summary.sh` | Test summary. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |
| `scripts/workload_partition.sh` | Workload partition. | Run during focused development of the named subsystem. | No, not by default. | No; contributors may run when relevant. |

## Historical

| Script | Purpose | When to run | CI runs it? | Maintainers only? |
| --- | --- | --- | --- | --- |
| `scripts/restore_runtime.sh` | Restore runtime. | Run only to reproduce preserved historical proof/archive flows. | No, not by default. | No; contributors may run when relevant. |
