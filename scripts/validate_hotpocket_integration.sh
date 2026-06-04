#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/hotpocket_integration_validation_report.txt"
ADAPTER="$ROOT/hotpocket/adapter/hotpocket_adapter.sh"
mkdir -p "$REPORT_DIR"

status_for_file() {
  local path="$1"
  if [[ -s "$path" ]]; then
    printf 'PASS'
  else
    printf 'FAIL'
  fi
}

require_grep() {
  local pattern="$1"
  local path="$2"
  grep -q "$pattern" "$path"
}

adapter_status="FAIL"
input_status="FAIL"
output_status="FAIL"
checkpoint_status="FAIL"
replay_status="FAIL"
settlement_status="FAIL"
status_status="FAIL"
overall="FAIL"

if [[ -x "$ADAPTER" ]]; then
  adapter_status="PASS"
fi

# Prepare protocol-node artifacts through the Protocol Node Appliance only.
bash scripts/node_init.sh >/dev/null
bash scripts/node_start.sh >/dev/null
bash scripts/node_checkpoint.sh >/dev/null
bash scripts/node_replay.sh >/dev/null
bash scripts/node_restore.sh >/dev/null

bash "$ADAPTER" init-layout
bash "$ADAPTER" ingest-input input-0001 2026-06-04T00:00:00Z hotpocket-runtime '{"action":"validate","sequence":1}'
input_file="$ROOT/hotpocket/input/input-0001.input.env"
input_root="$ROOT/hotpocket/input/input_root"
if require_grep '^Envelope Type: HotPocket Input$' "$input_file" \
  && require_grep '^Input Identifier: input-0001$' "$input_file" \
  && require_grep '^Serialization: canonical-field-order-v0.1$' "$input_file" \
  && [[ -s "$input_root" ]]; then
  input_status="PASS"
fi

input_root_value="$(cat "$input_root")"
bash "$ADAPTER" export-output output-0001 accepted PASS "$input_root_value"
output_file="$ROOT/hotpocket/output/output-0001.output.env"
output_root="$ROOT/hotpocket/output/output_root"
if require_grep '^Envelope Type: HotPocket Output$' "$output_file" \
  && require_grep '^Output Identifier: output-0001$' "$output_file" \
  && require_grep '^Status: PASS$' "$output_file" \
  && [[ -s "$output_root" ]]; then
  output_status="PASS"
fi

bash "$ADAPTER" export-checkpoint checkpoint-export-v0.1
checkpoint_file="$ROOT/hotpocket/checkpoint/checkpoint-export-v0.1.checkpoint.env"
checkpoint_root="$ROOT/hotpocket/checkpoint/checkpoint_export_root"
if require_grep '^Envelope Type: HotPocket Checkpoint Export$' "$checkpoint_file" \
  && require_grep '^Continuity Root: ' "$checkpoint_file" \
  && require_grep '^HotPocket Mutates State: false$' "$checkpoint_file" \
  && [[ -s "$checkpoint_root" ]]; then
  checkpoint_status="PASS"
fi

bash "$ADAPTER" export-replay replay-export-v0.1
replay_file="$ROOT/hotpocket/replay/replay-export-v0.1.replay.env"
replay_root="$ROOT/hotpocket/replay/replay_export_root"
if require_grep '^Envelope Type: HotPocket Replay Export$' "$replay_file" \
  && require_grep '^Replay Root: ' "$replay_file" \
  && require_grep '^HotPocket Mutates Replay Root: false$' "$replay_file" \
  && [[ -s "$replay_root" ]]; then
  replay_status="PASS"
fi

bash "$ADAPTER" export-settlement settlement-export-v0.1
settlement_file="$ROOT/hotpocket/settlement/settlement-export-v0.1.settlement.env"
settlement_root="$ROOT/hotpocket/settlement/settlement_export_root"
if require_grep '^Envelope Type: HotPocket Settlement Export$' "$settlement_file" \
  && require_grep '^Settlement Root: ' "$settlement_file" \
  && require_grep '^XRPL Execution: disabled$' "$settlement_file" \
  && [[ -s "$settlement_root" ]]; then
  settlement_status="PASS"
fi

bash "$ADAPTER" export-status
status_file="$ROOT/hotpocket/status/status-export-v0.1.status.env"
status_root="$ROOT/hotpocket/status/status_root"
if require_grep '^Envelope Type: HotPocket Status Export$' "$status_file" \
  && require_grep '^Node Status: ' "$status_file" \
  && require_grep '^Adapter Status: ready$' "$status_file" \
  && [[ -s "$status_root" ]]; then
  status_status="PASS"
fi

if [[ "$adapter_status" == PASS \
  && "$input_status" == PASS \
  && "$output_status" == PASS \
  && "$checkpoint_status" == PASS \
  && "$replay_status" == PASS \
  && "$settlement_status" == PASS \
  && "$status_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
HotPocket Integration Validation Report
Adapter: $adapter_status
Input Export: $input_status
Output Export: $output_status
Checkpoint Export: $checkpoint_status
Replay Export: $replay_status
Settlement Export: $settlement_status
Status Export: $status_status
HotPocket Integration Validation: $overall
REPORT_BODY

cat "$REPORT"
[[ "$overall" == PASS ]]
