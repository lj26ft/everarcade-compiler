#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/benchmarks/reports"
mkdir -p "$REPORT_DIR"

python3 - "$ROOT_DIR" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
dag = json.loads((root / "benchmarks/dag/scaling.json").read_text())
replay = json.loads((root / "benchmarks/replay/scaling.json").read_text())
wasm = json.loads((root / "benchmarks/wasm/scaling.json").read_text())

summary = {
    "profile_version": 1,
    "slowest_runtime_path": max(dag, key=lambda x: x["diagnostic_duration_ns"]),
    "dag_scaling": dag,
    "replay_scaling": replay,
    "wasm_scaling": wasm,
}

json_out = root / "benchmarks/reports/performance-report.json"
json_out.write_text(json.dumps(summary, indent=2) + "\n")

lines = [
    "# Deterministic Performance Report",
    "",
    "## DAG Scaling",
    "| Nodes | Execution | Receipts | State Diffs | Memory Bytes | Duration (ns) |",
    "|---:|---:|---:|---:|---:|---:|",
]
for row in dag:
    lines.append(f"| {row['dag_nodes']} | {row['execution_count']} | {row['receipt_count']} | {row['state_diff_count']} | {row['memory_bytes']} | {row['diagnostic_duration_ns']} |")

lines += [
    "",
    "## Replay Scaling",
    "| Ops | Archive Bytes | Verify (ns) | Reconstruct (ns) | Checkpoint Restore (ns) |",
    "|---:|---:|---:|---:|---:|",
]
for row in replay:
    lines.append(f"| {row['replay_operations']} | {row['archive_bytes']} | {row['verify_duration_ns']} | {row['reconstruct_duration_ns']} | {row['checkpoint_restore_duration_ns']} |")

lines += ["", "## WASM Overhead", "| Scenario | Calls | Fuel | Pages | Receipts | Duration (ns) |", "|---|---:|---:|---:|---:|---:|"]
for row in wasm:
    lines.append(f"| {row['scenario']} | {row['wasm_calls']} | {row['fuel_consumed']} | {row['memory_pages_touched']} | {row['receipt_count']} | {row['diagnostic_duration_ns']} |")

lines += ["", "## Estimated Evernode Sizing Implications", "- CPU headroom required for replay verification spikes.", "- Memory scales with state diff and replay reconstruction depth.", "- Archive growth should be monitored against retention policy."]

(root / "benchmarks/reports/performance-report.md").write_text("\n".join(lines) + "\n")
PY

echo "Generated benchmarks/reports/performance-report.json"
echo "Generated benchmarks/reports/performance-report.md"
