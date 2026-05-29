#!/usr/bin/env bash
set -euo pipefail

echo "protocol_readiness_validation=running mode=offline locked=true deterministic_packages=true open_registry=true"
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_protocol_readiness --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_package_generation_equivalence --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package execution-core --test creator_pipeline_runtime_tests test_protocol_readiness --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package execution-core --test creator_pipeline_runtime_tests test_package_generation_equivalence --offline --locked
echo "protocol_readiness_validation=ok"
