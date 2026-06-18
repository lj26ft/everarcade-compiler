#!/usr/bin/env node
const assert = require('node:assert/strict');
const { existsSync, mkdirSync, readFileSync, writeFileSync } = require('node:fs');
const { join, relative } = require('node:path');
const { canonicalHash, canonicalize } = require('../src/arena_vanguard');

const ROOT_KEYS = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'];
const REPORT_DIR = join(process.cwd(), 'reports', 'hotpocket-federation-certification');

const NODE_CANDIDATES = [
  { id: 'node1', aliases: ['node1', 'node-a'], state: ['node1/contract_fs/seed/state/state/arena-wrapper-state.json', 'reports/hotpocket-live/node-a/work/state/arena-wrapper-state.json', 'deployments/arena-vanguard-consensus/node-a/evernode/hotpocket/arena-wrapper-state.json'], journal: ['node1/contract_fs/seed/state/state/arena-hotpocket-journal.json', 'reports/hotpocket-live/node-a/work/state/arena-hotpocket-journal.json', 'deployments/arena-vanguard-consensus/node-a/evernode/journals/arena-hotpocket-journal.json'] },
  { id: 'node2', aliases: ['node2', 'node-b'], state: ['node2/contract_fs/seed/state/state/arena-wrapper-state.json', 'reports/hotpocket-live/node-b/work/state/arena-wrapper-state.json', 'deployments/arena-vanguard-consensus/node-b/evernode/hotpocket/arena-wrapper-state.json'], journal: ['node2/contract_fs/seed/state/state/arena-hotpocket-journal.json', 'reports/hotpocket-live/node-b/work/state/arena-hotpocket-journal.json', 'deployments/arena-vanguard-consensus/node-b/evernode/journals/arena-hotpocket-journal.json'] },
  { id: 'node3', aliases: ['node3', 'node-c'], state: ['node3/contract_fs/seed/state/state/arena-wrapper-state.json', 'reports/hotpocket-live/node-c/work/state/arena-wrapper-state.json', 'deployments/arena-vanguard-consensus/node-c/evernode/hotpocket/arena-wrapper-state.json'], journal: ['node3/contract_fs/seed/state/state/arena-hotpocket-journal.json', 'reports/hotpocket-live/node-c/work/state/arena-hotpocket-journal.json', 'deployments/arena-vanguard-consensus/node-c/evernode/journals/arena-hotpocket-journal.json'] }
];

function readJson(path) { return JSON.parse(readFileSync(path, 'utf8')); }
function firstExisting(paths) { return paths.map((p) => join(process.cwd(), p)).find(existsSync); }
function latestCommitments(snapshot) {
  const commitments = snapshot?.state?.commitments;
  assert.ok(Array.isArray(commitments) && commitments.length > 0, 'state snapshot must include at least one commitment');
  const latest = commitments.at(-1);
  for (const key of ROOT_KEYS) assert.ok(typeof latest[key] === 'string' && latest[key].length > 0, `latest commitment missing ${key}`);
  return latest;
}
function status(ok) { return ok ? 'PASS' : 'FAIL'; }
function writeJson(name, value) { writeFileSync(join(REPORT_DIR, name), `${JSON.stringify(value, null, 2)}\n`); }
function allEqual(values) { return new Set(values).size === 1; }

function loadNodes() {
  return NODE_CANDIDATES.map((node) => {
    const statePath = firstExisting(node.state);
    assert.ok(statePath, `missing state file for ${node.id}; checked ${node.state.join(', ')}`);
    const journalPath = firstExisting(node.journal);
    assert.ok(journalPath, `missing journal file for ${node.id}; checked ${node.journal.join(', ')}`);
    const snapshot = readJson(statePath);
    const journal = readJson(journalPath);
    assert.ok(Array.isArray(journal), `${node.id} journal must be an array`);
    const latest = latestCommitments(snapshot);
    return { id: node.id, aliases: node.aliases, statePath, journalPath, latest, journal };
  });
}

function rootArtifact(nodes, key) {
  const values = Object.fromEntries(nodes.map((node) => [node.id, node.latest[key]]));
  return { schema: `everarcade.hotpocket.federation.${key}.v0.1`, generated_at: '1970-01-01T00:00:00.000Z', nodes: values, match: allEqual(Object.values(values)), canonical: Object.values(values)[0] };
}

function replayPersistedJournal(journal) {
  assert.ok(journal.length > 0, 'journal must include replay entries');
  let expectedBefore = canonicalize({ tick: 0, players: {}, combat_events: [], last_sequence: {}, commitments: [] });
  const checks = [];
  for (const [index, entry] of journal.entries()) {
    assert.equal(entry.sequence, index + 1, `journal sequence mismatch at offset ${index}`);
    assert.equal(entry.canonical_state_before, expectedBefore, `journal state chain mismatch at sequence ${entry.sequence}`);
    const journalBase = { ...entry };
    delete journalBase.journal_hash;
    delete journalBase.hotpocket;
    const journalHash = canonicalHash(journalBase);
    const hashMatch = journalHash === entry.journal_hash;
    const stateAfter = JSON.parse(entry.canonical_state_after);
    const latest = stateAfter.commitments?.at(-1);
    assert.ok(latest, `journal sequence ${entry.sequence} missing commitment`);
    checks.push({ sequence: entry.sequence, journal_hash: journalHash, journal_hash_match: hashMatch, roots: latest });
    expectedBefore = entry.canonical_state_after;
  }
  return { commitments: checks.at(-1).roots, entries_verified: checks.length, journal_hash_chain: checks, ok: checks.every((check) => check.journal_hash_match) };
}

function verifyReplay(nodes) {
  const canonicalLive = nodes[0].latest;
  const nodeResults = nodes.map((node) => {
    const replayed = replayPersistedJournal(node.journal);
    const comparisons = Object.fromEntries(ROOT_KEYS.map((key) => [key, { live: node.latest[key], replayed: replayed.commitments[key], match: node.latest[key] === replayed.commitments[key], matches_canonical_live: replayed.commitments[key] === canonicalLive[key] }]));
    return { node: node.id, state_path: relative(process.cwd(), node.statePath), journal_path: relative(process.cwd(), node.journalPath), journal_entries: node.journal.length, replay_method: 'persisted canonical journal from genesis', journal_hashes_verified: replayed.ok, live: node.latest, replayed: replayed.commitments, comparisons, ok: replayed.ok && ROOT_KEYS.every((key) => comparisons[key].match && comparisons[key].matches_canonical_live) };
  });
  return { schema: 'everarcade.hotpocket.federation.replay-verification.v0.1', replay_from: 'genesis', roots_verified: ROOT_KEYS, nodes: nodeResults, ok: nodeResults.every((node) => node.ok) };
}

function reportText(rootArtifacts, replay) {
  const lines = [
    'EverArcade Federation Replay Certification',
    '',
    'World: Arena Vanguard',
    'Nodes Verified: 3',
    '',
    `State Root Match: ${status(rootArtifacts.state_root.match)}`,
    `Receipt Root Match: ${status(rootArtifacts.receipt_root.match)}`,
    `World Hash Match: ${status(rootArtifacts.world_hash.match)}`,
    `Continuity Root Match: ${status(rootArtifacts.continuity_root.match)}`,
    '',
    `Replay Verification: ${status(replay.ok)}`,
    '',
    'Result:',
    `LIVE HOTPOCKET FEDERATION CERTIFICATION: ${status(Object.values(rootArtifacts).every((artifact) => artifact.match) && replay.ok)}`,
    ''
  ];
  return lines.join('\n');
}

function main() {
  mkdirSync(REPORT_DIR, { recursive: true });
  const nodes = loadNodes();
  const rootArtifacts = Object.fromEntries(ROOT_KEYS.map((key) => [key, rootArtifact(nodes, key)]));
  writeJson('federation_state_roots.json', rootArtifacts.state_root);
  writeJson('federation_receipt_roots.json', rootArtifacts.receipt_root);
  writeJson('federation_world_hashes.json', rootArtifacts.world_hash);
  writeJson('federation_continuity_roots.json', rootArtifacts.continuity_root);
  const replay = verifyReplay(nodes);
  writeJson('replay_verification.json', replay);
  const text = reportText(rootArtifacts, replay);
  writeFileSync(join(REPORT_DIR, 'certification_report.txt'), text);
  const ok = Object.values(rootArtifacts).every((artifact) => artifact.match) && replay.ok;
  console.log(text.trim());
  if (!ok) process.exit(1);
}

main();
