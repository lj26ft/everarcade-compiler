#!/usr/bin/env node
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { ArenaHotPocketRuntime, canonicalHash, canonicalize, defaultPaths, genesisState, replayJournal } from '../src/runtime.mjs';

const root = process.env.EVERARCADE_REPO_ROOT || process.cwd();
const deploymentRoot = join(root, 'deployments/arena-vanguard-consensus');
const reportRoot = join(root, 'reports/consensus');
const nodes = ['node-a', 'node-b', 'node-c'];
const commitmentKeys = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'];
const inputSequence = Object.freeze([
  { action: 'join', player: 'player-1' },
  { action: 'join', player: 'player-2' },
  { action: 'move', player: 'player-1', direction: 'east' },
  { action: 'move', player: 'player-2', direction: 'west' },
  { action: 'attack', player: 'player-1', target: 'player-2' }
]);

function shaFile(path) { return createHash('sha256').update(readFileSync(path)).digest('hex'); }
function ensureDir(path) { mkdirSync(path, { recursive: true }); }
function writeJson(path, value) { ensureDir(dirname(path)); writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`); }
function writeText(path, value) { ensureDir(dirname(path)); writeFileSync(path, value); }
function cleanNode(node) { rmSync(join(deploymentRoot, node, 'evernode'), { recursive: true, force: true }); rmSync(join(reportRoot, node), { recursive: true, force: true }); ensureDir(join(reportRoot, node)); }
function rootsOnly(commitment) { return Object.fromEntries(commitmentKeys.map((key) => [key, commitment[key]])); }
function rowsMarkdown(rows) { return rows.map((r) => `| ${r.node} | ${r.lclSeqNo} | ${r.state_root} | ${r.receipt_root} | ${r.world_hash} | ${r.continuity_root} |`).join('\n'); }
function compareHistories(histories) {
  const divergence = [];
  for (let i = 0; i < histories['node-a'].length; i += 1) {
    for (const key of commitmentKeys) {
      const expected = histories['node-a'][i][key];
      for (const node of nodes.slice(1)) {
        const actual = histories[node][i]?.[key];
        if (actual !== expected) divergence.push({ round: histories['node-a'][i].lclSeqNo, node, key, expected, actual, first_divergence_point: i + 1 });
      }
    }
  }
  return divergence;
}
function projectionFrom(state) { return { players: state.players, combat_events: state.combat_events, tick: state.tick }; }

const artifactPaths = {
  contract: join(root, 'hotpocket-arena-wrapper/contract/hotpocket-adapter.mjs'),
  runtime: join(root, 'hotpocket-arena-wrapper/src/runtime.mjs'),
  genesis: join(deploymentRoot, 'node-a/genesis/genesis.json'),
  package: join(root, 'hotpocket-arena-wrapper/package.json')
};
const expectedGenesis = canonicalHash(genesisState());
const preflight = Object.fromEntries(nodes.map((node) => {
  const nodeRoot = join(deploymentRoot, node);
  return [node, {
    genesis_hash: canonicalHash(JSON.parse(readFileSync(join(nodeRoot, 'genesis/genesis.json'), 'utf8'))),
    package_hash: shaFile(artifactPaths.package),
    contract_hash: shaFile(join(nodeRoot, 'contract/hotpocket-adapter.mjs')),
    runtime_hash: shaFile(join(nodeRoot, 'runtime/runtime.mjs'))
  }];
}));

const preflightOk = nodes.every((node) => preflight[node].genesis_hash === expectedGenesis)
  && ['package_hash', 'contract_hash', 'runtime_hash'].every((key) => nodes.every((node) => preflight[node][key] === preflight['node-a'][key]));

const histories = {};
const replayResults = {};
const journalComparisons = {};
const projectionResults = {};
const failureInjection = {};

for (const node of nodes) {
  cleanNode(node);
  const runtime = new ArenaHotPocketRuntime(defaultPaths(join(deploymentRoot, node))).load();
  histories[node] = [];
  inputSequence.forEach((input, index) => {
    const lclSeqNo = index + 1;
    const result = runtime.processAtRound({ ...input, hotpocket: { round: lclSeqNo, source: 'canonical-test-stream' } }, lclSeqNo);
    histories[node].push({ node, lclSeqNo, input, ...rootsOnly(result.commitments) });
  });

  const invalidBefore = rootsOnly(runtime.verify().live);
  let invalidRejected = false;
  try { runtime.processAtRound({ action: 'move', player: 'player-1', direction: 'up', hotpocket: { round: 6, source: 'failure-injection' } }, 6); } catch { invalidRejected = true; }
  const invalidAfter = rootsOnly(runtime.verify().live);
  const duplicate = runtime.processAtRound({ ...inputSequence.at(-1), hotpocket: { round: 7, source: 'duplicate-injection' } }, 7);
  const restarted = new ArenaHotPocketRuntime(defaultPaths(join(deploymentRoot, node))).load();
  const replayed = replayJournal(restarted.journal);
  replayResults[node] = { ok: restarted.verify().ok, live: rootsOnly(restarted.verify().live), replayed: rootsOnly(replayed.commitments) };
  failureInjection[node] = { invalid_input_rejected: invalidRejected, invalid_input_preserved_commitments: canonicalize(invalidBefore) === canonicalize(invalidAfter), duplicate_input_root: rootsOnly(duplicate.commitments), restart_replay_ok: restarted.verify().ok };
  journalComparisons[node] = { journal_hashes: restarted.journal.map((entry) => entry.journal_hash), receipt_hashes: restarted.receipts.map((receipt) => receipt.receipt_hash), tick_sequence: restarted.journal.map((entry) => entry.round) };
  projectionResults[node] = projectionFrom(restarted.state);

  writeJson(join(reportRoot, node, 'rounds.json'), histories[node]);
  writeJson(join(reportRoot, node, 'journal-summary.json'), journalComparisons[node]);
  writeJson(join(reportRoot, node, 'replay.json'), replayResults[node]);
  writeJson(join(reportRoot, node, 'projection.json'), projectionResults[node]);
}

const divergence = compareHistories(histories);
const journalOk = nodes.slice(1).every((node) => canonicalize(journalComparisons[node]) === canonicalize(journalComparisons['node-a']));
const replayOk = nodes.every((node) => replayResults[node].ok && commitmentKeys.every((key) => replayResults[node].live[key] === replayResults[node].replayed[key]));
const projectionOk = nodes.slice(1).every((node) => canonicalize(projectionResults[node]) === canonicalize(projectionResults['node-a']));
const failureOk = nodes.every((node) => failureInjection[node].invalid_input_rejected && failureInjection[node].invalid_input_preserved_commitments && failureInjection[node].restart_replay_ok)
  && nodes.slice(1).every((node) => canonicalize(failureInjection[node].duplicate_input_root) === canonicalize(failureInjection['node-a'].duplicate_input_root));
const ok = preflightOk && divergence.length === 0 && journalOk && replayOk && projectionOk && failureOk;

const rows = nodes.flatMap((node) => histories[node]);
const report = `# Arena Vanguard Multi-Node Consensus Report\n\n- Node count: ${nodes.length}\n- Input sequence: ${inputSequence.map((i) => `\`${canonicalize(i)}\``).join(', ')}\n- Round count: ${inputSequence.length}\n- Genesis/package/contract validation: ${preflightOk ? 'PASS' : 'FAIL'}\n- Replay validation: ${replayOk ? 'PASS' : 'FAIL'}\n- Journal comparison: ${journalOk ? 'PASS' : 'FAIL'}\n- Projection validation: ${projectionOk ? 'PASS' : 'FAIL'}\n- Divergence detection: ${divergence.length === 0 ? 'PASS (no divergence)' : 'FAIL'}\n- Failure injection: ${failureOk ? 'PASS' : 'FAIL'}\n\n## Commitment History\n\n| Node | Round | state_root | receipt_root | world_hash | continuity_root |\n| --- | ---: | --- | --- | --- | --- |\n${rowsMarkdown(rows)}\n\n## Divergence Results\n\n\`${JSON.stringify(divergence, null, 2)}\`\n`;
writeJson(join(reportRoot, 'preflight.json'), preflight);
writeJson(join(reportRoot, 'divergence.json'), divergence);
writeJson(join(reportRoot, 'failure-injection.json'), failureInjection);
writeText(join(reportRoot, 'arena-vanguard-consensus-report.md'), report);

console.log(`Arena Vanguard Multi-Node Consensus: ${ok ? 'PASS' : 'FAIL'}`);
console.log(`Report: ${relative(root, join(reportRoot, 'arena-vanguard-consensus-report.md'))}`);
process.exit(ok ? 0 : 1);
