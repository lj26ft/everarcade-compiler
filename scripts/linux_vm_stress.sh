#!/usr/bin/env bash
set -euo pipefail

STATE_DIR=".everarcade-stress"
TMP_FIXTURE="$(mktemp)"
trap 'rm -f "$TMP_FIXTURE"' EXIT

rm -rf "$STATE_DIR"
mkdir -p "$STATE_DIR"

cargo run -p everarcade-host -- generate-fixture --output "$TMP_FIXTURE"
for _ in $(seq 1 5); do
  cargo run -p everarcade-host -- run --package "$TMP_FIXTURE" --state "$STATE_DIR"
  cargo run -p everarcade-host -- verify --state "$STATE_DIR"
done
