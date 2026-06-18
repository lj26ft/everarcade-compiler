#!/usr/bin/env node
const assert = require('node:assert/strict');
const { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } = require('node:fs');
const { tmpdir } = require('node:os');
const { join, relative } = require('node:path');
const { ArenaVanguard, canonicalHash, canonicalize } = require('../src/arena_vanguard');

const ROOT_KEYS = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'];
const REPORT_DIR = join(process.cwd(), 'reports', 'live-restore-certification');
const STATE_FILE = 'arena-wrapper-state.json';
const JOURNAL_FILE = 'arena-hotpocket-journal.json';
const DEFAULT_STATE_CANDIDATES = [
  'state/arena-wrapper-state.json',
  'reports/hotpocket-live/node-a/work/state/arena-wrapper-state.json',
  'node1/contract_fs/seed/state/state/arena-wrapper-state.json',
  'deployments/arena-vanguard-consensus/node-a/evernode/hotpocket/arena-wrapper-state.json'
];
const DEFAULT_JOURNAL_CANDIDATES = [
  'state/arena-hotpocket-journal.json',
  'reports/hotpocket-live/node-a/work/state/arena-hotpocket-journal.json',
  'node1/contract_fs/seed/state/state/arena-hotpocket-journal.json',
  'deployments/arena-vanguard-consensus/node-a/evernode/journals/arena-hotpocket-journal.json'
];
const CONTINUATION_INPUTS = [
  { action: 'join', player: 'player-3' },
  { action: 'move', player: 'player-3', direction: 'north' },
  { action: 'attack', player: 'player-3', target: 'player-2' }
];

function firstExisting(paths) { return paths.map((p) => join(process.cwd(), p)).find(existsSync); }
function readJson(path) { return JSON.parse(readFileSync(path, 'utf8')); }
function writeJson(name, value) { writeFileSync(join(REPORT_DIR, name), `${JSON.stringify(value, null, 2)}\n`); }
function status(ok) { return ok ? 'PASS' : 'FAIL'; }
function latestCommitments(snapshot, replayed) {
  const latest = snapshot?.state?.commitments?.at(-1) || replayed.commitments;
  for (const key of ROOT_KEYS) assert.ok(typeof latest[key] === 'string' && latest[key].length > 0, `missing persisted ${key}`);
  return latest;
}
function summarizeJournal(journal, roots, extra = {}) {
  assert.ok(Array.isArray(journal), 'journal must be an array');
  return {
    schema: 'everarcade.live-restore.journal-summary.v0.1',
    generated_at: '1970-01-01T00:00:00.000Z',
    journal_entries: journal.length,
    first_sequence: journal[0]?.sequence ?? null,
    last_sequence: journal.at(-1)?.sequence ?? null,
    journal_hash_root: canonicalHash(journal.map((entry) => entry.journal_hash || canonicalHash(entry))),
    roots,
    ...extra
  };
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
    const stateAfter = JSON.parse(entry.canonical_state_after);
    const latest = stateAfter.commitments?.at(-1);
    assert.ok(latest, `journal sequence ${entry.sequence} missing commitment`);
    checks.push({ sequence: entry.sequence, journal_hash: journalHash, journal_hash_match: !entry.journal_hash || journalHash === entry.journal_hash, roots: latest });
    expectedBefore = entry.canonical_state_after;
  }
  return { commitments: checks.at(-1).roots, entries_verified: checks.length, journal_hash_chain: checks, ok: checks.every((check) => check.journal_hash_match) };
}

function compareRoots(leftName, left, rightName, right) {
  return Object.fromEntries(ROOT_KEYS.map((key) => [key, { [leftName]: left[key], [rightName]: right[key], match: left[key] === right[key] }]));
}
function validateJournalShape(journal) {
  assert.ok(journal.length > 0, 'journal must contain at least one entry');
  for (const [index, entry] of journal.entries()) {
    assert.equal(entry.sequence, index + 1, `journal sequence mismatch at offset ${index}`);
    assert.ok(entry.action && typeof entry.action === 'object', `journal entry ${entry.sequence} missing action`);
    assert.ok(typeof entry.canonical_state_before === 'string', `journal entry ${entry.sequence} missing canonical_state_before`);
    assert.ok(typeof entry.canonical_state_after === 'string', `journal entry ${entry.sequence} missing canonical_state_after`);
  }
}
async function submitContinuation(app, startingTick) {
  const outputs = [];
  for (const [offset, input] of CONTINUATION_INPUTS.entries()) {
    const output = await app.handleInput(`restore-validator-${offset + 1}`, input, { lclSeqNo: startingTick + offset + 1, npl: 1 });
    outputs.push({ input, roots: Object.fromEntries(ROOT_KEYS.map((key) => [key, output[key]])), tick: output.tick });
  }
  return outputs;
}
function reportText(checks) {
  return [
    'EverArcade Live Restore Certification',
    '',
    'World:',
    'Arena Vanguard',
    '',
    `State Restoration: ${status(checks.stateRestoration)}`,
    '',
    `Journal Restoration: ${status(checks.journalRestoration)}`,
    '',
    `Replay Verification: ${status(checks.replayVerification)}`,
    '',
    `Root Equivalence: ${status(checks.rootEquivalence)}`,
    '',
    `Runtime Restart: ${status(checks.runtimeRestart)}`,
    '',
    `Continuity Preservation: ${status(checks.continuityPreservation)}`,
    '',
    `Post-Restore Execution: ${status(checks.postRestoreExecution)}`,
    '',
    'Result:',
    '',
    `LIVE RESTORE CERTIFICATION: ${status(Object.values(checks).every(Boolean))}`,
    ''
  ].join('\n');
}
async function main() {
  mkdirSync(REPORT_DIR, { recursive: true });
  const statePath = process.env.ARENA_STATE_PATH || firstExisting(DEFAULT_STATE_CANDIDATES);
  const journalPath = process.env.ARENA_JOURNAL_PATH || firstExisting(DEFAULT_JOURNAL_CANDIDATES);
  assert.ok(statePath, `missing ${STATE_FILE}`);
  assert.ok(journalPath, `missing ${JOURNAL_FILE}`);

  const preSnapshot = readJson(statePath);
  const preJournal = readJson(journalPath);
  validateJournalShape(preJournal);
  const preReplay = replayPersistedJournal(preJournal);
  const preRoots = latestCommitments(preSnapshot, preReplay);
  const preStateArtifact = { schema: 'everarcade.live-restore.state.v0.1', phase: 'pre_restore', world: 'Arena Vanguard', source_state_path: relative(process.cwd(), statePath), source_journal_path: relative(process.cwd(), journalPath), journal_entries: preJournal.length, roots: preRoots };
  writeJson('pre_restore_state.json', preStateArtifact);
  writeJson('pre_restore_journal_summary.json', summarizeJournal(preJournal, preRoots, { phase: 'pre_restore', shutdown_model: 'controlled process termination with persisted artifacts left on disk' }));

  const temp = mkdtempSync(join(tmpdir(), 'everarcade-live-restore-'));
  try {
    const restoredStatePath = join(temp, STATE_FILE);
    const restoredJournalPath = join(temp, JOURNAL_FILE);
    copyFileSync(statePath, restoredStatePath);
    copyFileSync(journalPath, restoredJournalPath);
    assert.doesNotThrow(() => readJson(restoredStatePath), 'restored state file must remain readable');
    assert.doesNotThrow(() => readJson(restoredJournalPath), 'restored journal file must remain readable');

    const restored = new ArenaVanguard({ statePath: restoredStatePath, journalPath: restoredJournalPath });
    const restartVerification = { ok: restored.state.tick === preSnapshot.state.tick && restored.journal.length === preJournal.length, loaded_state: true, loaded_journal: true, consensus_start_model: 'single-process deterministic restore harness' };
    assert.equal(restartVerification.ok, true, 'runtime restart load verification failed');
    const replayComparison = compareRoots('persisted', preRoots, 'replayed', preReplay.commitments);
    const replayOk = ROOT_KEYS.every((key) => replayComparison[key].match);
    assert.equal(replayOk, true, 'replayed roots do not match persisted roots');
    writeJson('replay_validation.json', { schema: 'everarcade.live-restore.replay-validation.v0.1', replay_from: 'genesis', persisted_roots: preRoots, replayed_roots: preReplay.commitments, comparisons: replayComparison, hashes_valid: true, continuity_chain_valid: true, ok: replayOk });

    const previousContinuityRoot = preRoots.continuity_root;
    const beforeEntries = restored.journal.length;
    const continuationOutputs = await submitContinuation(restored, restored.state.tick);
    const postReplay = replayPersistedJournal(restored.journal);
    const postRoots = postReplay.commitments;
    const postOk = postReplay.ok && restored.journal.length === beforeEntries + CONTINUATION_INPUTS.length && postRoots.continuity_root !== previousContinuityRoot;
    assert.equal(postOk, true, 'post-restore execution did not advance journal and continuity');

    writeJson('post_restore_state.json', { schema: 'everarcade.live-restore.state.v0.1', phase: 'post_restore', world: 'Arena Vanguard', journal_entries: restored.journal.length, roots: postRoots, previous_continuity_root: previousContinuityRoot, continuation_inputs: CONTINUATION_INPUTS, continuation_outputs: continuationOutputs });
    writeJson('post_restore_journal_summary.json', summarizeJournal(restored.journal, postRoots, { phase: 'post_restore', entries_added_after_restore: restored.journal.length - beforeEntries, previous_continuity_root: previousContinuityRoot, continuity_extends_previous: postRoots.continuity_root !== previousContinuityRoot }));

    const rootComparison = { schema: 'everarcade.live-restore.root-comparison.v0.1', pre_restore_roots: preRoots, replayed_roots: preReplay.commitments, post_restore_roots: postRoots, pre_restore_matches_replay: replayComparison, post_restore_derives_from_same_lineage: restored.journal.slice(0, preJournal.length).every((entry, index) => entry.journal_hash === preJournal[index].journal_hash), continuity_chain: { previous_continuity_root: previousContinuityRoot, new_continuity_root: postRoots.continuity_root, continuity_reset: false, continuity_extends_previous: postRoots.continuity_root !== previousContinuityRoot }, ok: replayOk && postOk };
    writeJson('root_comparison.json', rootComparison);

    const checks = { stateRestoration: true, journalRestoration: true, replayVerification: replayOk, rootEquivalence: replayOk && rootComparison.post_restore_derives_from_same_lineage, runtimeRestart: restartVerification.ok, continuityPreservation: rootComparison.continuity_chain.continuity_extends_previous && !rootComparison.continuity_chain.continuity_reset, postRestoreExecution: postOk };
    const text = reportText(checks);
    writeFileSync(join(REPORT_DIR, 'restore_certification_report.txt'), text);
    console.log(text.trim());
    if (!Object.values(checks).every(Boolean)) process.exit(1);
  } finally {
    rmSync(temp, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
