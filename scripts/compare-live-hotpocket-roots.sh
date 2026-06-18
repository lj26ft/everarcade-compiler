#!/usr/bin/env bash
set -euo pipefail
ROOT="${EVERARCADE_REPO_ROOT:-$(pwd)}"
REPORT_DIR="$ROOT/reports/live-hotpocket-cluster"
node --input-type=module - "$REPORT_DIR" <<'NODE'
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
const reportDir=process.argv[2]; const nodes=['node-a','node-b','node-c']; const keys=['state_root','receipt_root','world_hash','continuity_root'];
const rounds=Object.fromEntries(nodes.map((n)=>[n,JSON.parse(readFileSync(join(reportDir,n,'rounds.json'),'utf8'))])); const divergence=[];
for(let i=0;i<rounds['node-a'].length;i+=1){for(const key of keys){const expected=rounds['node-a'][i][key]; for(const node of nodes.slice(1)){const actual=rounds[node][i]?.[key]; if(actual!==expected) divergence.push({node,round:rounds['node-a'][i].ctx_lclSeqNo,root_type:key,expected_root:expected,actual_root:actual,first_divergent_journal_entry:i+1});}}}
const out={schema:'everarcade.live-hotpocket-cluster.root-comparison.v0.1',status:divergence.length?'FAIL':'PASS',divergence}; writeFileSync(join(reportDir,'root-comparison.json'),`${JSON.stringify(out,null,2)}\n`);
if(divergence.length){ for(const d of divergence) console.error(`Mismatch node=${d.node} round=${d.round} root=${d.root_type} expected=${d.expected_root} actual=${d.actual_root} first_divergent_journal_entry=${d.first_divergent_journal_entry}`); process.exit(1); }
console.log('Live HotPocket cluster roots match for node-a, node-b, and node-c.');
NODE
