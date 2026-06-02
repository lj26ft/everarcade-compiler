#!/usr/bin/env bash
set -euo pipefail
CARGO_BUILD_JOBS=1 cargo test -p execution-core --test arena_vanguard_playable_tests test_player_move --offline --locked
CARGO_BUILD_JOBS=1 cargo test -p execution-core --test arena_vanguard_playable_tests test_player_attack --offline --locked
node --test frontend/tests/run_frontend_integration_tests.mjs
printf 'Runtime connectivity validation complete\n'
