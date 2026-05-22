#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST="$ROOT/dist"
MANIFEST="$ROOT/runtime/manifests/runtime-release-manifest.json"
PACKAGE="$DIST/everarcade-runtime-appliance-v0.1.0.tar.gz"

[[ -f "$PACKAGE" ]] || { echo "missing package $PACKAGE" >&2; exit 1; }
[[ -f "$DIST/runtime.sha256" ]] || { echo "missing $DIST/runtime.sha256" >&2; exit 1; }
[[ -f "$DIST/runtime-manifest.sha256" ]] || { echo "missing $DIST/runtime-manifest.sha256" >&2; exit 1; }
[[ -f "$MANIFEST" ]] || { echo "missing runtime manifest $MANIFEST" >&2; exit 1; }
[[ -f "$ROOT/runtime/config/runtime.toml" ]] || { echo "missing runtime artifact runtime/config/runtime.toml" >&2; exit 1; }
[[ -f "$ROOT/runtime/world/status.txt" ]] || { echo "missing runtime artifact runtime/world/status.txt" >&2; exit 1; }
[[ -f "$ROOT/runtime/manifests/runtime-artifacts.sha256" ]] || { echo "missing runtime artifact hash list" >&2; exit 1; }

( cd "$ROOT" && sha256sum -c dist/runtime.sha256 )
( cd "$ROOT" && sha256sum -c dist/runtime-manifest.sha256 )
( cd "$ROOT" && sha256sum -c runtime/manifests/runtime-artifacts.sha256 >/dev/null )

echo "verify_runtime_package=ok"
