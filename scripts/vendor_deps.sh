#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

mkdir -p .cargo
BACKUP=""
if [[ -f .cargo/config.toml ]]; then
  BACKUP=".cargo/config.toml.bak"
  cp .cargo/config.toml "$BACKUP"
fi

cleanup() {
  if [[ -n "$BACKUP" && -f "$BACKUP" ]]; then
    mv "$BACKUP" .cargo/config.toml
  fi
}
trap cleanup EXIT

rm -f .cargo/config.toml
cargo generate-lockfile
cargo vendor --locked vendor > .cargo/config.toml
cat >> .cargo/config.toml <<'CFG'

[net]
offline = true
CFG

rm -f "$BACKUP"
trap - EXIT

echo "dependency_vendor=ok"
