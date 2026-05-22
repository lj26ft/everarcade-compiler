#!/usr/bin/env bash
set -euo pipefail

cargo test -p execution-core journal_hash_is_replay_stable -- --nocapture
echo "execution replay verification passed"
