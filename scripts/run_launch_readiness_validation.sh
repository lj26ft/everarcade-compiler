#!/usr/bin/env bash
set -euo pipefail

echo "launch_readiness_validation=running mode=offline locked=true publish_deploy_live=true replay_continuity=preserved authority_mutation=rejected"
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_creator_pipeline_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_template_generation_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_publish_workflow_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_creator_readiness
CARGO_BUILD_JOBS=1 cargo test --package execution-core --test creator_pipeline_runtime_tests test_replay_safe_pipeline --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package execution-core --test creator_pipeline_runtime_tests test_authority_mutation_rejection --offline --locked
echo "launch_readiness_validation=ok"
