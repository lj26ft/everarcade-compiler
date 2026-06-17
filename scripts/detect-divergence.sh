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
for (let i = 0; i < rounds['node-a'].length; i += 1) {
  for (const key of keys) {
    const expected = rounds['node-a'][i][key];
    for (const node of nodes.slice(1)) {
      const actual = rounds[node][i]?.[key];
      if (actual !== expected) {
        console.error(`round number: ${rounds['node-a'][i].lclSeqNo}`);
        console.error(`key: ${key}`);
        console.error(`expected root: ${expected}`);
        console.error(`actual root: ${actual}`);
        console.error(`first divergence point: ${i + 1}`);
        process.exit(1);
      }
    }
  }
}
console.log('No divergence detected across consensus roots.');
NODE
