#!/usr/bin/env node
const assert = require('node:assert/strict');
const { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } = require('node:fs');
const { tmpdir } = require('node:os');
const { join } = require('node:path');
const { ArenaVanguard, canonicalHash, canonicalize, replayJournal } = require('../src/arena_vanguard');

const REPORT_DIR = join(process.cwd(), 'reports', 'hotpocket-migration');
const ROOT_KEYS = ['state_root', 'receipt_root', 'continuity_root', 'world_hash'];
const SOURCE_INPUTS = [
  { action: 'join', player: 'player-1' },
  { action: 'join', player: 'player-2' },
  { action: 'move', player: 'player-1', direction: 'east' },
  { action: 'move', player: 'player-2', direction: 'west' },
  { action: 'attack', player: 'player-1', target: 'player-2' }
];
const CONTINUATION_INPUTS = [
  { action: 'join', player: 'player-3' },
  { action: 'move', player: 'player-3', direction: 'north' },
  { action: 'attack', player: 'player-3', target: 'player-2' }
];
function writeJson(name, value) { writeFileSync(join(REPORT_DIR, name), `${JSON.stringify(value, null, 2)}\n`); }
function readJson(path) { return JSON.parse(readFileSync(path, 'utf8')); }
function rootsOf(snapshot) { const roots = snapshot.state.commitments.at(-1); assert.ok(roots, 'snapshot missing roots'); return roots; }
function pickRoots(roots) { return Object.fromEntries(['tick', ...ROOT_KEYS].map((key) => [key, roots[key]])); }
function compare(a, b) { return Object.fromEntries(ROOT_KEYS.map((key) => [key, { source: a[key], destination: b[key], match: a[key] === b[key] }])); }
function allRootsMatch(a, b) { return ROOT_KEYS.every((key) => a[key] === b[key]); }
function journalHashRoot(journal) { return canonicalHash(journal.map((entry) => entry.journal_hash || canonicalHash(entry))); }
function verifyJournal(journal) {
  let expectedBefore = canonicalize({ tick: 0, players: {}, combat_events: [], last_sequence: {}, commitments: [] });
  const checks = [];
  for (const [index, entry] of journal.entries()) {
    assert.equal(entry.sequence, index + 1, `journal sequence mismatch at offset ${index}`);
    assert.equal(entry.canonical_state_before, expectedBefore, `continuity ordering mismatch at sequence ${entry.sequence}`);
    const base = { ...entry };
    delete base.journal_hash;
    delete base.hotpocket;
    const computed = canonicalHash(base);
    checks.push({ sequence: entry.sequence, journal_hash: entry.journal_hash, computed_journal_hash: computed, journal_hash_match: computed === entry.journal_hash, receipt_hash: entry.receipt_hash, roots: pickRoots(entry) });
    expectedBefore = entry.canonical_state_after;
  }
  const replayed = replayJournal(journal);
  return { entries_verified: checks.length, journal_hash_root: journalHashRoot(journal), journal_hashes_valid: checks.every((check) => check.journal_hash_match), receipt_ordering_valid: checks.every((check, index) => check.sequence === index + 1), continuity_ordering_valid: true, replay_roots: pickRoots(replayed.commitments), checks };
}
async function drive(app, inputs, userPrefix, startTick = 0) {
  const outputs = [];
  for (const [offset, input] of inputs.entries()) {
    const output = await app.handleInput(`${userPrefix}-${offset + 1}`, input, { lclSeqNo: startTick + offset + 1, npl: 1, readonly: false });
    outputs.push({ input, tick: output.tick, roots: pickRoots(output.commitments) });
  }
  return outputs;
}
function replayArtifact(name, journal, liveRoots) {
  const replayed = replayJournal(journal);
  const comparisons = Object.fromEntries(ROOT_KEYS.map((key) => [key, { live: liveRoots[key], replayed: replayed.commitments[key], match: liveRoots[key] === replayed.commitments[key] }]));
  return { schema: `everarcade.hotpocket-migration.${name}.v0.1`, replay_from: 'genesis', journal_entries: journal.length, live_roots: pickRoots(liveRoots), replay_roots: pickRoots(replayed.commitments), comparisons, ok: ROOT_KEYS.every((key) => comparisons[key].match) };
}
function report(checks) {
  const status = (ok) => ok ? 'PASS' : 'FAIL';
  const ok = Object.values(checks).every(Boolean);
  return ['EverArcade Live World Migration Certification', '', 'World:', 'Arena Vanguard', '', `Source Destination Root Equivalence: ${status(checks.rootEquivalence)}`, '', `World Hash Equivalence: ${status(checks.worldHashEquivalence)}`, '', `Replay Verification: ${status(checks.replayVerification)}`, '', `Journal Verification: ${status(checks.journalVerification)}`, '', `Continuity Preservation: ${status(checks.continuityPreservation)}`, '', `Post-Migration Continuation: ${status(checks.postMigrationContinuation)}`, '', `No Reset Or Fork: ${status(checks.noResetOrFork)}`, '', 'Result:', '', `LIVE WORLD MIGRATION CERTIFICATION: ${status(ok)}`, ''].join('\n');
}
async function main() {
  rmSync(REPORT_DIR, { recursive: true, force: true });
  mkdirSync(REPORT_DIR, { recursive: true });
  const sourceTemp = mkdtempSync(join(tmpdir(), 'everarcade-migration-source-'));
  const destTemp = mkdtempSync(join(tmpdir(), 'everarcade-migration-destination-'));
  try {
    const source = new ArenaVanguard({ statePath: join(sourceTemp, 'source-state.json'), journalPath: join(sourceTemp, 'source-journal.json') });
    await drive(source, SOURCE_INPUTS, 'source-operator');
    const sourceSnapshot = readJson(source.statePath);
    const sourceJournal = readJson(source.journalPath);
    const sourceRoots = rootsOf(sourceSnapshot);
    writeJson('source-state.json', sourceSnapshot);
    writeJson('source-journal.json', sourceJournal);
    writeJson('source-roots.json', { schema: 'everarcade.hotpocket-migration.roots.v0.1', operator: 'source', world: 'Arena Vanguard', journal_entries: sourceJournal.length, roots: pickRoots(sourceRoots), state_hash: canonicalHash(sourceSnapshot.state), journal_hash_root: journalHashRoot(sourceJournal) });

    const migrationPackage = { schema: 'everarcade.hotpocket-migration.package.v0.1', world: 'Arena Vanguard', created_at: '1970-01-01T00:00:00.000Z', source_operator: { role: 'exporter', node_identity: null, host_configuration: null }, portability_guarantees: { self_contained: true, source_filesystem_paths_required: false, source_node_identity_required: false, source_host_configuration_required: false }, manifest: { canonicalizer: 'runtime/arena_hotpocket/src/arena_vanguard.js canonicalize()', state_hash: canonicalHash(sourceSnapshot.state), snapshot_hash: canonicalHash(sourceSnapshot), journal_hash_root: journalHashRoot(sourceJournal), roots: pickRoots(sourceRoots), entries: sourceJournal.length }, canonical_state: sourceSnapshot, canonical_journal: sourceJournal };
    writeJson('migration-package.json', migrationPackage);

    const destStatePath = join(destTemp, 'destination-state.json');
    const destJournalPath = join(destTemp, 'destination-journal.json');
    writeFileSync(destStatePath, `${JSON.stringify(migrationPackage.canonical_state, null, 2)}\n`);
    writeFileSync(destJournalPath, `${JSON.stringify(migrationPackage.canonical_journal, null, 2)}\n`);
    const destination = new ArenaVanguard({ statePath: destStatePath, journalPath: destJournalPath });
    const destSnapshot = readJson(destStatePath);
    const destJournal = readJson(destJournalPath);
    const destRoots = rootsOf(destSnapshot);
    writeJson('destination-state.json', destSnapshot);
    writeJson('destination-journal.json', destJournal);
    writeJson('destination-roots.json', { schema: 'everarcade.hotpocket-migration.roots.v0.1', operator: 'destination', world: 'Arena Vanguard', journal_entries: destJournal.length, roots: pickRoots(destRoots), state_hash: canonicalHash(destSnapshot.state), journal_hash_root: journalHashRoot(destJournal) });

    const replaySource = replayArtifact('replay-source', sourceJournal, sourceRoots);
    const replayDestination = replayArtifact('replay-destination', destJournal, destRoots);
    writeJson('replay-source.json', replaySource);
    writeJson('replay-destination.json', replayDestination);

    const sourceVerify = verifyJournal(sourceJournal);
    const destVerify = verifyJournal(destJournal);
    const continuityVerification = { schema: 'everarcade.hotpocket-migration.continuity-verification.v0.1', source: sourceVerify, destination: destVerify, journal_hashes_valid: sourceVerify.journal_hashes_valid && destVerify.journal_hashes_valid, receipt_ordering_valid: sourceVerify.receipt_ordering_valid && destVerify.receipt_ordering_valid, continuity_ordering_valid: sourceVerify.continuity_ordering_valid && destVerify.continuity_ordering_valid, replay_roots_identical: allRootsMatch(sourceVerify.replay_roots, destVerify.replay_roots), continuity_extends_rather_than_forks: true, ok: false };

    const previousRoots = pickRoots(destRoots);
    const beforeJournalLength = destination.journal.length;
    const continuationOutputs = await drive(destination, CONTINUATION_INPUTS, 'destination-operator', destination.state.tick);
    const postSnapshot = readJson(destStatePath);
    const postJournal = readJson(destJournalPath);
    const postRoots = rootsOf(postSnapshot);
    const lineagePreserved = postJournal.slice(0, sourceJournal.length).every((entry, index) => entry.journal_hash === sourceJournal[index].journal_hash);
    const advances = ROOT_KEYS.every((key) => postRoots[key] !== destRoots[key]);
    continuityVerification.post_migration = { previous_continuity_root: destRoots.continuity_root, new_continuity_root: postRoots.continuity_root, continuity_extends_previous: postJournal[sourceJournal.length]?.canonical_state_before === sourceJournal.at(-1).canonical_state_after, lineage_preserved: lineagePreserved, entries_added: postJournal.length - beforeJournalLength };
    continuityVerification.ok = continuityVerification.journal_hashes_valid && continuityVerification.receipt_ordering_valid && continuityVerification.continuity_ordering_valid && continuityVerification.replay_roots_identical && continuityVerification.post_migration.continuity_extends_previous && lineagePreserved;
    writeJson('continuity-verification.json', continuityVerification);

    const rootComparison = { schema: 'everarcade.hotpocket-migration.root-comparison.v0.1', source_roots: pickRoots(sourceRoots), destination_roots: pickRoots(destRoots), replay_roots: { source: replaySource.replay_roots, destination: replayDestination.replay_roots }, post_migration_roots: pickRoots(postRoots), source_destination: compare(sourceRoots, destRoots), replay_live: { source: replaySource.comparisons, destination: replayDestination.comparisons }, continuation: { previous_roots: previousRoots, continuation_inputs: CONTINUATION_INPUTS, continuation_outputs: continuationOutputs, roots_advanced: advances, state_reset: postSnapshot.state.tick <= destSnapshot.state.tick, fork_detected: !lineagePreserved, replay_after_continuation: replayArtifact('post-migration-replay', postJournal, postRoots) }, ok: allRootsMatch(sourceRoots, destRoots) && replaySource.ok && replayDestination.ok && advances && lineagePreserved };
    writeJson('root-comparison.json', rootComparison);

    const checks = { rootEquivalence: allRootsMatch(sourceRoots, destRoots), worldHashEquivalence: sourceRoots.world_hash === destRoots.world_hash, replayVerification: replaySource.ok && replayDestination.ok, journalVerification: continuityVerification.ok, continuityPreservation: continuityVerification.post_migration.continuity_extends_previous, postMigrationContinuation: advances && postJournal.length === beforeJournalLength + CONTINUATION_INPUTS.length, noResetOrFork: !rootComparison.continuation.state_reset && !rootComparison.continuation.fork_detected };
    const text = report(checks);
    writeFileSync(join(REPORT_DIR, 'certification-report.txt'), text);
    console.log(text.trim());
    if (!Object.values(checks).every(Boolean)) process.exit(1);
  } finally {
    rmSync(sourceTemp, { recursive: true, force: true });
    rmSync(destTemp, { recursive: true, force: true });
  }
}
main().catch((error) => { console.error(error); process.exit(1); });
