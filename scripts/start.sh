#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MODE="background"
PROFILE="false"
for arg in "$@"; do
  [[ "$arg" == "--foreground" ]] && MODE="foreground"
  [[ "$arg" == "--profile" ]] && PROFILE="true"
done
"$ROOT/scripts/bootstrap.sh"
log(){ printf '[runtime][%s][%s][deterministic=true]\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1"; }
log "config-load"
"$ROOT/bin/everarcade-cli" runtime-snapshot "$ROOT/runtime/config" > "$ROOT/runtime/manifests/runtime-snapshot.json"
log "scheduler-init"
mkdir -p "$ROOT/runtime/replay/latest" "$ROOT/runtime/state/journal"
echo '{}' > "$ROOT/runtime/replay/latest/frame-0000.json"
log "host-launch"
if [[ "$PROFILE" == "true" ]]; then
  mkdir -p "$ROOT/runtime/diagnostics"
  printf '{"component":"start.sh","event":"profile_enabled","deterministic":true}
' >> "$ROOT/runtime/diagnostics/runtime-profile.jsonl"
fi
if [[ "$MODE" == "foreground" ]]; then exec "$ROOT/bin/everarcade-host"; else "$ROOT/bin/everarcade-host"; fi
