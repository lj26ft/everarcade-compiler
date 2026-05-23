#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
log(){ printf '[bootstrap][%s][%s][deterministic=true]\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1"; }
for d in bin runtime/config runtime/games runtime/worlds runtime/replay runtime/archives runtime/manifests runtime/state logs scripts docs; do [ -d "$ROOT/$d" ] || { log "missing:$d"; exit 1; }; done
for b in everarcade-host everarcade-cli everarcade-validator; do [ -x "$ROOT/bin/$b" ] || { log "missing-binary:$b"; exit 1; }; done
[ -f "$ROOT/MANIFEST.json" ] && [ -f "$ROOT/RELEASE_ROOT" ] || { log 'missing-manifest'; exit 1; }
for c in runtime federation storage replay topology evernode; do [ -f "$ROOT/runtime/config/$c.toml" ] || { log "missing-config:$c"; exit 1; }; done
log 'bootstrap-ok'
