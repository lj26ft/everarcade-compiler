# Live Deployment Readiness

Classification: Scaffold

Validation command: CARGO_BUILD_JOBS=1 cargo test -p execution-core --test live_deployment_tests --offline --locked

Result: PASS

Single-, two-, five-, and ten-node federation deployment paths are represented in the provider validation harness.

Remaining limitation: no external EverNode hosts are leased by the automated tests; this remains a deterministic local provider-path certification scaffold.
