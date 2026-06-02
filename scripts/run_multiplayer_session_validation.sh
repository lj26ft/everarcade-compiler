#!/usr/bin/env bash
set -euo pipefail
CARGO_BUILD_JOBS=1 cargo test -p execution-core --test arena_vanguard_playable_tests test_multiplayer_join --offline --locked
printf 'Multiplayer session validation complete\n'
