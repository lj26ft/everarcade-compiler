#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

bash "$ROOT/scripts/hash_runtime_artifacts.sh"
bash "$ROOT/scripts/verify_runtime_package.sh"
bash "$ROOT/scripts/verify_replay_determinism.sh"
bash "$ROOT/scripts/verify_runtime_manifest_signature.sh"

[[ -f "$ROOT/runtime/manifests/runtime-artifacts.sha256" ]] || { echo "missing runtime-artifacts.sha256" >&2; exit 1; }
[[ -f "$ROOT/runtime/manifests/replay.sha256" ]] || { echo "missing replay.sha256" >&2; exit 1; }

( cd "$ROOT" && sha256sum -c runtime/manifests/runtime-artifacts.sha256 >/dev/null )
( cd "$ROOT" && sha256sum -c runtime/manifests/replay.sha256 >/dev/null )

echo "cross_machine_repro_check=ok"
