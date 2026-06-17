#!/usr/bin/env bash
set -euo pipefail
ROOT="${EVERARCADE_REPO_ROOT:-$(pwd)}"
REPORT_DIR="$ROOT/reports/consensus"
node --input-type=module - "$REPORT_DIR" <<'NODE'
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
const reportDir = process.argv[2];
const nodes = ['node-a', 'node-b', 'node-c'];
const keys = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'];
const rounds = Object.fromEntries(nodes.map((node) => [node, JSON.parse(readFileSync(join(reportDir, node, 'rounds.json'), 'utf8'))]));
let ok = true;
for (let i = 0; i < rounds['node-a'].length; i += 1) {
  for (const key of keys) {
    const expected = rounds['node-a'][i][key];
    for (const node of nodes.slice(1)) {
      const actual = rounds[node][i]?.[key];
      if (actual !== expected) {
        ok = false;
        console.error(`Mismatch round=${rounds['node-a'][i].lclSeqNo} key=${key} expected=${expected} node=${node} actual=${actual}`);
      }
    }
  }
}
if (ok) console.log('Consensus roots match for node-a, node-b, and node-c.');
process.exit(ok ? 0 : 1);
NODE
