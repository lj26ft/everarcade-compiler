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


if [[ -z "$(find "$ROOT/world/journal" -maxdepth 1 -type f 2>/dev/null)" ]]; then
  GENESIS_ROOT=$("$BIN" replay-world --world-root "$ROOT/world" | awk -F= '/^state_root=/{print $2}')
  mkdir -p "$ROOT/world/genesis" "$ROOT/world/timeline" "$ROOT/world/world_state"
  cat > "$ROOT/world/genesis/genesis.json" <<JSON
{
  "genesis_root": "${GENESIS_ROOT}",
  "protocol_version": 1,
  "topology_epoch": 0
}
JSON
fi

echo "runtime_start=ok root=$ROOT state=$STATE"
"$BIN" verify-journal --world-root "$ROOT/world" | tee -a "$LOG"
"$BIN" replay-world --world-root "$ROOT/world" | tee -a "$LOG"
"$BIN" status --state "$STATE" | tee -a "$LOG"
