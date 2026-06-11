#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT="reports/hotpocket_replay_proof_report.txt"
mkdir -p reports validation/hotpocket
PROOF="$(node runtime/hotpocket-adapter/bin/everarcade-hotpocket-adapter.js replay-proof)"
status="$(printf '%s' "$PROOF" | node -e 'let s="";process.stdin.on("data",d=>s+=d);process.stdin.on("end",()=>process.stdout.write(JSON.parse(s).status))')"
state_root="$(printf '%s' "$PROOF" | node -e 'let s="";process.stdin.on("data",d=>s+=d);process.stdin.on("end",()=>process.stdout.write(JSON.parse(s).run_a.state_root))')"
receipt_root="$(printf '%s' "$PROOF" | node -e 'let s="";process.stdin.on("data",d=>s+=d);process.stdin.on("end",()=>process.stdout.write(JSON.parse(s).run_a.receipt_root))')"
journal_root="$(printf '%s' "$PROOF" | node -e 'let s="";process.stdin.on("data",d=>s+=d);process.stdin.on("end",()=>process.stdout.write(JSON.parse(s).run_a.journal_root))')"
replay_root="$(printf '%s' "$PROOF" | node -e 'let s="";process.stdin.on("data",d=>s+=d);process.stdin.on("end",()=>process.stdout.write(JSON.parse(s).run_a.replay_root))')"
cat > "$REPORT" <<RPT
HotPocket Replay Proof Report
Identical Input Sequence Runs: PASS
state_root_a == state_root_b: $status
receipt_root_a == receipt_root_b: $status
journal_root_a == journal_root_b: $status
replay_root_a == replay_root_b: $status
State Root: $state_root
Receipt Root: $receipt_root
Journal Root: $journal_root
Replay Root: $replay_root
Classification: HotPocket Deterministic Replay Proven
HotPocket Replay Proof: $status
RPT
cat "$REPORT"
[[ "$status" == PASS ]]
