#!/usr/bin/env bash
set -euo pipefail

STATE_DIR="$(mktemp -d /tmp/everarcade-recovery.XXXXXX)"
TMP_FIXTURE="$(mktemp)"
trap 'rm -rf "$STATE_DIR"; rm -f "$TMP_FIXTURE"' EXIT


cargo run -p everarcade-host -- generate-fixture --output "$TMP_FIXTURE"
cargo run -p everarcade-host -- run --package "$TMP_FIXTURE" --state "$STATE_DIR"
cargo run -p everarcade-host -- repair-manifest --state "$STATE_DIR"
cargo run -p everarcade-host -- rebuild-index --state "$STATE_DIR"
cargo run -p everarcade-host -- verify --state "$STATE_DIR"
