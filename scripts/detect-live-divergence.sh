#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-reports/hotpocket-live}"
node - "$ROOT" <<'NODE'
const fs = require('fs'); const path = require('path');
const root = process.argv[2]; const nodes = ['node-a','node-b','node-c'];
const data = Object.fromEntries(nodes.map((n)=>[n, JSON.parse(fs.readFileSync(path.join(root,n,'rounds.json'),'utf8'))]));
const keys = ['state_root','receipt_root','world_hash','continuity_root'];
for (let i=0;i<Math.max(...nodes.map((n)=>data[n].length));i++) {
  const exp = data['node-a'][i]?.commitments || {};
  for (const n of nodes) for (const k of keys) {
    const actual = data[n][i]?.commitments?.[k];
    if (actual !== exp[k]) {
      const out = { first_divergent_round: i + 1, node: n, key: k, expected_root: exp[k] || null, actual_root: actual || null };
      fs.writeFileSync(path.join(root,'divergence.json'), JSON.stringify({ schema:'everarcade.hotpocket-live.divergence.v0.1', divergent:true, ...out, status:'FAIL' }, null, 2) + '\n');
      console.log(JSON.stringify(out, null, 2)); process.exit(1);
    }
  }
}
fs.writeFileSync(path.join(root,'divergence.json'), JSON.stringify({ schema:'everarcade.hotpocket-live.divergence.v0.1', divergent:false, message:'No divergence detected', status:'PASS' }, null, 2) + '\n');
console.log('No divergence detected');
NODE
