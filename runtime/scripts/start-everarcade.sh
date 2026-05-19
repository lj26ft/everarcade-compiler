#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="$ROOT/bin/everarcade-host"
STATE="$ROOT/world/state"
LOG="$ROOT/logs/runtime.log"

required_dirs=(bin config world logs contracts vendor scripts world/state world/checkpoints world/journal world/receipts)
for d in "${required_dirs[@]}"; do
  [[ -d "$ROOT/$d" ]] || { echo "runtime_start=failed missing_dir=$d" >&2; exit 1; }
done

required_files=("$BIN" "$ROOT/config/runtime.toml" "$ROOT/config/federation.toml" "$ROOT/config/storage.toml")
for f in "${required_files[@]}"; do
  [[ -f "$f" ]] || { echo "runtime_start=failed missing_file=$f" >&2; exit 1; }
done

mkdir -p "$ROOT/logs" "$ROOT/world/state" "$ROOT/world/checkpoints" "$ROOT/world/journal" "$ROOT/world/receipts"

if [[ ! -f "$STATE/node.json" ]]; then
  "$BIN" init --state "$STATE" >/dev/null
fi

echo "runtime_start=ok root=$ROOT state=$STATE"
"$BIN" status --state "$STATE" | tee -a "$LOG"
