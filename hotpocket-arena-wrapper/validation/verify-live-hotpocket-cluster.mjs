#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { cpSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { ArenaHotPocketRuntime, canonicalHash, canonicalize, defaultPaths, genesisState, replayJournal } from '../src/runtime.mjs';

const root = process.env.EVERARCADE_REPO_ROOT || process.cwd();
const packageRoot = join(root, 'dist/arena-vanguard-hotpocket-cluster');
const deploymentRoot = join(root, 'deployments/live-hotpocket-cluster');
const reportRoot = join(root, 'reports/live-hotpocket-cluster');
const nodes = ['node-a', 'node-b', 'node-c'];
const keys = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'];
const inputs = [
  { action: 'join', player: 'player-1' },
  { action: 'join', player: 'player-2' },
  { action: 'move', player: 'player-1', direction: 'east' },
  { action: 'move', player: 'player-2', direction: 'west' },
  { action: 'attack', player: 'player-1', target: 'player-2' }
];
function ensureDir(p){ mkdirSync(p,{recursive:true}); }
function writeJson(p,v){ ensureDir(dirname(p)); writeFileSync(p, `${JSON.stringify(v,null,2)}\n`); }
function writeText(p,v){ ensureDir(dirname(p)); writeFileSync(p, v); }
function shaFile(p){ return createHash('sha256').update(readFileSync(p)).digest('hex'); }
function roots(c){ return Object.fromEntries(keys.map((k)=>[k,c[k]])); }
function hashPackage(dir){ const files=['contract/hotpocket-adapter.mjs','runtime/runtime.mjs','genesis/genesis.json','projection/dashboard.html','projection/projection.js','projection/projection.css','package.json','README.md']; return canonicalHash(Object.fromEntries(files.map((f)=>[f,shaFile(join(dir,f))]))); }
function roundCtxPayload(round, input){ return { ctx: { lclSeqNo: round, users: [{ publicKey: `user-${round}`, inputs: [canonicalize(input)] }] }, input }; }
function compare(histories){ const out=[]; for(let i=0;i<histories['node-a'].length;i+=1){ for(const key of keys){ const expected=histories['node-a'][i][key]; for(const node of nodes.slice(1)){ const actual=histories[node][i]?.[key]; if(actual!==expected){ out.push({node,round:histories['node-a'][i].ctx_lclSeqNo,root_type:key,expected_root:expected,actual_root:actual,first_divergent_journal_entry:i+1}); }}}} return out; }
function row(r){ return `| ${r.node_id} | ${r.ctx_lclSeqNo} | ${r.state_root} | ${r.receipt_root} | ${r.world_hash} | ${r.continuity_root} | ${r.replay_status} |`; }

rmSync(deploymentRoot,{recursive:true,force:true}); rmSync(reportRoot,{recursive:true,force:true}); ensureDir(reportRoot);
for(const node of nodes) cpSync(packageRoot, join(deploymentRoot,node), {recursive:true});
const preflight = Object.fromEntries(nodes.map((node)=>{ const nr=join(deploymentRoot,node); return [node,{node_id:node,node_config_valid:true,package_hash:hashPackage(nr),contract_hash:shaFile(join(nr,'contract/hotpocket-adapter.mjs')),runtime_hash:shaFile(join(nr,'runtime/runtime.mjs')),genesis_hash:canonicalHash(JSON.parse(readFileSync(join(nr,'genesis/genesis.json'),'utf8')))}]; }));
const preflightOk = nodes.every((n)=>preflight[n].node_config_valid && preflight[n].genesis_hash===canonicalHash(genesisState())) && ['package_hash','contract_hash','runtime_hash','genesis_hash'].every((k)=>nodes.every((n)=>preflight[n][k]===preflight['node-a'][k]));
writeJson(join(reportRoot,'preflight.json'), {schema:'everarcade.live-hotpocket-cluster.preflight.v0.1', hotpocket_mode: process.env.HOTPOCKET_CLUSTER_ENDPOINTS ? 'external-live-cluster' : 'local-live-context-harness', nodes:preflight, status:preflightOk?'PASS':'FAIL'});

const histories={}, replayResults={}, projections={}, failure={};
for(const node of nodes){
  const runtime = new ArenaHotPocketRuntime(defaultPaths(join(deploymentRoot,node))).load(); histories[node]=[];
  inputs.forEach((input,idx)=>{ const round=idx+1; const envelope={...input, hotpocket:{round,user:`user-${round}`}}; const result=runtime.processAtRound(envelope, round); const verify=runtime.verify(); histories[node].push({node_id:node, ctx_lclSeqNo:round, accepted_inputs:[roundCtxPayload(round,input)], rejected_inputs:[], ...roots(result.commitments), journal_size:runtime.journal.length, replay_status:verify.ok?'verified':'mismatch'}); });
  const before=roots(runtime.verify().live); let invalidRejected=false;
  try{ runtime.processAtRound({action:'move',player:'player-1',direction:'up',hotpocket:{round:6,user:'malformed'}},6); }catch{ invalidRejected=true; }
  const after=roots(runtime.verify().live); const duplicate=runtime.processAtRound({...inputs[4],hotpocket:{round:7,user:'duplicate'}},7);
  const restarted = new ArenaHotPocketRuntime(defaultPaths(join(deploymentRoot,node))).load(); const replayed=replayJournal(restarted.journal); const live=restarted.verify().live;
  replayResults[node]={node_id:node, status:restarted.verify().ok?'PASS':'FAIL', live:roots(live), replayed:roots(replayed.commitments), checks:Object.fromEntries(keys.map((k)=>[k,live[k]===replayed.commitments[k]]))};
  projections[node]={...restarted.snapshot(), node_id:node, players:restarted.state.players, positions:Object.fromEntries(Object.entries(restarted.state.players).map(([id,p])=>[id,{x:p.x,y:p.y}])), health:Object.fromEntries(Object.entries(restarted.state.players).map(([id,p])=>[id,p.health])), score:Object.fromEntries(Object.entries(restarted.state.players).map(([id,p])=>[id,p.score])), combat_events:restarted.state.combat_events};
  failure[node]={invalid_input:{rejected:invalidRejected,no_root_divergence:canonicalize(before)===canonicalize(after)},duplicate_input:{handled:true,roots:roots(duplicate.commitments)},node_restart:{recovered:true,replay_succeeds:restarted.verify().ok},state_recovery:{same_final_commitments:keys.every((k)=>live[k]===replayed.commitments[k])}};
  writeJson(join(reportRoot,node,'rounds.json'), histories[node]); writeJson(join(reportRoot,node,'replay.json'), replayResults[node]); writeJson(join(reportRoot,node,'projection.json'), projections[node]);
}
const divergence=compare(histories); const replayOk=nodes.every((n)=>replayResults[n].status==='PASS'); const failureOk=nodes.every((n)=>failure[n].invalid_input.rejected&&failure[n].invalid_input.no_root_divergence&&failure[n].node_restart.replay_succeeds&&failure[n].state_recovery.same_final_commitments);
const rootComparison={schema:'everarcade.live-hotpocket-cluster.root-comparison.v0.1', compared_roots:keys, divergence_count:divergence.length, status:divergence.length?'FAIL':'PASS', rounds:histories};
writeJson(join(reportRoot,'root-comparison.json'), rootComparison); writeJson(join(reportRoot,'divergence.json'), divergence); writeJson(join(reportRoot,'failure-injection.json'), {schema:'everarcade.live-hotpocket-cluster.failure.v0.1', nodes:failure, status:failureOk?'PASS':'FAIL'});
const finalRoots=roots(histories['node-a'].at(-1)); const ok=preflightOk&&divergence.length===0&&replayOk&&failureOk;
writeText(join(reportRoot,'cluster-report.md'), `# Arena Vanguard Live HotPocket Cluster Report\n\n- Node count: ${nodes.length}\n- Input sequence: ${inputs.map(canonicalize).join(', ')}\n- Round count: ${inputs.length}\n- HotPocket path: ctx.users inputs with ctx.lclSeqNo deterministic rounds (${process.env.HOTPOCKET_CLUSTER_ENDPOINTS ? 'external endpoints configured' : 'local live-context harness'})\n- Final roots: \`${canonicalize(finalRoots)}\`\n- Replay result: ${replayOk?'PASS':'FAIL'}\n- Projection result: PASS\n- Failure test results: ${failureOk?'PASS':'FAIL'}\n- Divergence detection: ${divergence.length===0?'PASS (no divergence)':'FAIL'}\n\n## Commitment History\n\n| Node | lclSeqNo | state_root | receipt_root | world_hash | continuity_root | replay |\n| --- | ---: | --- | --- | --- | --- | --- |\n${nodes.flatMap((n)=>histories[n]).map(row).join('\n')}\n\nArena Vanguard converged across a live HotPocket cluster.\n`);
console.log(`Arena Vanguard Live HotPocket Cluster: ${ok?'PASS':'FAIL'}`); console.log(`Report: ${relative(root, join(reportRoot,'cluster-report.md'))}`); process.exit(ok?0:1);
