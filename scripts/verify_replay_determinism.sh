#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FRAME="$ROOT/runtime/replay/latest/frame-0001.json"

cargo run -p everarcade-cli -- start >/dev/null
[[ -f "$FRAME" ]] || { echo "missing replay frame: $FRAME" >&2; exit 1; }
FIRST="$(sha256sum "$FRAME" | awk '{print $1}')"

cargo run -p everarcade-cli -- start >/dev/null
[[ -f "$FRAME" ]] || { echo "missing replay frame after rerun: $FRAME" >&2; exit 1; }
SECOND="$(sha256sum "$FRAME" | awk '{print $1}')"

if [[ "$FIRST" != "$SECOND" ]]; then
  echo "replay_determinism=failed first=$FIRST second=$SECOND" >&2
  exit 1
fi

echo "replay_determinism=ok frame=runtime/replay/latest/frame-0001.json sha256=$FIRST"
