#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d)"
PROFILE="false"
for arg in "$@"; do [[ "$arg" == "--profile" ]] && PROFILE="true"; done

"$ROOT/scripts/preflight_vendor.sh"
tar -xzf "$ROOT/dist/everarcade-runtime-linux-x86_64.tar.gz" -C "$TMP"
cd "$TMP/everarcade-runtime"
./scripts/bootstrap.sh
./scripts/validate.sh
./scripts/start.sh --foreground >/tmp/everarcade-runtime-start.log 2>&1 &
PID=$!
sleep 2
if kill -0 "$PID" >/dev/null 2>&1; then
  kill "$PID"
fi
./scripts/shutdown.sh || true

if [[ "$PROFILE" == "true" ]]; then
  mkdir -p "$ROOT/target/everarcade-profile"
  printf '{"component":"validate_clean_vm_bootstrap.sh","event":"profile_enabled","deterministic":true}
' > "$ROOT/target/everarcade-profile/bootstrap-profile.jsonl"
fi
