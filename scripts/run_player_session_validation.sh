#!/usr/bin/env bash
set -euo pipefail
CARGO_BUILD_JOBS=1 cargo test -p execution-core --test arena_vanguard_playable_tests test_player_join --offline --locked
CARGO_BUILD_JOBS=1 cargo test -p execution-core --test arena_vanguard_playable_tests test_disconnect_reconnect --offline --locked
printf 'Player session validation complete\n'
