#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
bash scripts/build_release_tarball.sh
bash scripts/verify_release.sh
bash scripts/validate_runtime.sh
bash scripts/restore_runtime.sh
