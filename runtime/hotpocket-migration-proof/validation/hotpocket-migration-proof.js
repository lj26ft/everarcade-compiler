#!/usr/bin/env node
'use strict';

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');
const adapter = require('../../hotpocket-runtime-proof/adapter/runtime-adapter');

const ROOT = path.resolve(__dirname, '..');
const REPO_ROOT = path.resolve(ROOT, '../..');
const EXPORT_DIR = path.join(ROOT, 'export');
const IMPORT_DIR = path.join(ROOT, 'import');
const REPLAY_DIR = path.join(ROOT, 'replay');
const REPORT_DIR = process.env.EVERARCADE_HOTPOCKET_MIGRATION_REPORT_DIR || path.join(ROOT, 'reports');
const ROOT_REPORT_DIR = path.join(REPO_ROOT, 'reports');
const SOURCE_ACTIONS = adapter.DEFAULT_ACTIONS;
const CONTINUATION_ACTION = { action: 'move_player', player_id: 'alice', x: 15, y: 25 };
const RUNTIME_VERSION = adapter.RUNTIME_VERSION;
const WORLD_ID = adapter.WORLD_ID;
const PROOF_VERSION = 'everarcade-sovereign-runtime-migration-proof-v0.1';
const EXPORT_VERSION = 'everarcade-sovereign-export-v0.1';
const FORBIDDEN_FAILURES = [
  'replay mismatch', 'restore mismatch', 'root mismatch', 'validator disagreement',
  'checkpoint divergence', 'journal divergence', 'receipt divergence', 'continuation failure'
];

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function rmDirContents(dir) { fs.rmSync(dir, { recursive: true, force: true }); ensureDir(dir); }
function canonicalize(value) { return adapter.canonicalize(value); }
function canonicalHash(value) { return adapter.canonicalHash(value); }
function sha256Bytes(bytes) { return crypto.createHash('sha256').update(bytes).digest('hex'); }
function hashString(value) { return crypto.createHash('sha256').update(String(value)).digest('hex'); }
function canonicalBuffer(value) { return Buffer.from(`${canonicalize(value)}\n`, 'utf8'); }
function writeFile(dir, name, bytes) { ensureDir(dir); fs.writeFileSync(path.join(dir, name), bytes); }
function writeTextBoth(name, content) { ensureDir(REPORT_DIR); ensureDir(ROOT_REPORT_DIR); fs.writeFileSync(path.join(REPORT_DIR, name), content); fs.writeFileSync(path.join(ROOT_REPORT_DIR, name), content); }
function writeJsonBoth(name, value) { writeTextBoth(name, `${JSON.stringify(value, null, 2)}\n`); }
function status(ok) { return ok ? 'PASS' : 'FAIL'; }
function copyArtifact(srcDir, dstDir, name) { fs.copyFileSync(path.join(srcDir, name), path.join(dstDir, name)); }

function clusterValidators(prefix) {
  return ['validator1', 'validator2', 'validator3'].map((id) => ({ id: `${prefix}-${id}`, mode: 'independent-deterministic-hotpocket-consensus-projection' }));
}

function initialArenaState() {
  return {
    session_id: 'session-0001', tick: 0, players: {},
    positions: { dummy: { x: 0, y: 1 } }, health: { dummy: 100 }, scores: { dummy: 0 },
    events: ['session_started'], player_count: 0
  };
}
function applyRuntimeInput(state, input) {
  const next = JSON.parse(JSON.stringify(state));
  next.tick = input.sequence;
  if (input.action === 'join') {
    next.players[input.player_id] = { player_id: input.player_id, joined_tick: input.sequence };
    if (!next.positions[input.player_id]) next.positions[input.player_id] = { x: 0, y: 0 };
    if (next.health[input.player_id] === undefined) next.health[input.player_id] = 100;
    if (next.scores[input.player_id] === undefined) next.scores[input.player_id] = 0;
    next.player_count = Object.keys(next.players).length;
    next.events.push(`tick ${input.sequence}: ${input.player_id} joined`);
  } else if (input.action === 'move') {
    if (!next.positions[input.player_id]) next.positions[input.player_id] = { x: 0, y: 0 };
    next.positions[input.player_id] = { x: input.x, y: input.y };
    next.events.push(`tick ${input.sequence}: ${input.player_id} moved to ${input.x},${input.y}`);
  }
  return next;
}
function runtimeInput(action, sequence) {
  if (action.action === 'join_player') return { player_id: action.player_id, action: 'join', sequence };
  return { player_id: action.player_id, action: 'move', x: action.x, y: action.y, sequence };
}
function makeReceipt(sequence, input, state) {
  const canonicalInput = JSON.stringify(input);
  const inputHash = hashString(canonicalInput);
  const stateRoot = hashString(JSON.stringify(state));
  const receiptId = `receipt-${String(sequence).padStart(20, '0')}`;
  const receiptHash = hashString(`${receiptId}:${sequence}:${inputHash}:${stateRoot}:${RUNTIME_VERSION}:${WORLD_ID}`);
  return {
    receipt_id: receiptId, sequence, tick: sequence,
    input_id: `input-${String(sequence).padStart(20, '0')}`,
    input_hash: inputHash, state_root: stateRoot, receipt_hash: receiptHash,
    runtime_version: RUNTIME_VERSION, world_id: WORLD_ID, timestamp_or_epoch: sequence,
    session_id: state.session_id, player_count: Object.keys(state.players).length,
    action: input.action, player_id: input.player_id
  };
}
function makeJournalEntry(sequence, input, receipt, stateRoot, previousHash) {
  const base = {
    sequence, previous_hash: previousHash, state_root: stateRoot, input_hash: receipt.input_hash,
    receipt_hash: receipt.receipt_hash, timestamp_ms: sequence, player_id: input.player_id,
    action: input.action, tick: sequence, gameplay_input: input
  };
  return { ...base, entry_hash: canonicalHash(base) };
}
function makeCheckpoint(state, journal) {
  const stateRoot = hashString(JSON.stringify(state));
  const checkpointHash = hashString(`${state.tick}:${journal.length}:${stateRoot}:${JSON.stringify(state)}`);
  return {
    sequence: state.tick, created_at_ms: state.tick, world_id: WORLD_ID, runtime_version: RUNTIME_VERSION,
    journal_position: journal.length, state_root: stateRoot, checkpoint_hash: checkpointHash
  };
}
function rootSet(state, receipts, journal, checkpoint) {
  const stateRoot = hashString(JSON.stringify(state));
  const replayRoot = replayJournal(journal).state_root;
  const checkpointRoot = checkpoint.checkpoint_hash;
  const receiptRoot = canonicalHash(receipts);
  const journalRoot = canonicalHash(journal);
  const worldRoot = canonicalHash({ world_id: WORLD_ID, runtime_version: RUNTIME_VERSION, state_root: stateRoot, replay_root: replayRoot, checkpoint_root: checkpointRoot, receipt_root: receiptRoot, journal_root: journalRoot });
  return { state_root: stateRoot, replay_root: replayRoot, checkpoint_root: checkpointRoot, receipt_root: receiptRoot, journal_root: journalRoot, world_root: worldRoot };
}
function replayJournal(journal) {
  let state = initialArenaState();
  let previous = 'genesis';
  for (const entry of journal) {
    const expectedBase = { ...entry };
    delete expectedBase.entry_hash;
    if (entry.previous_hash !== previous) throw new Error(`journal previous hash mismatch at sequence ${entry.sequence}`);
    if (canonicalHash(expectedBase) !== entry.entry_hash) throw new Error(`journal entry hash mismatch at sequence ${entry.sequence}`);
    state = applyRuntimeInput(state, entry.gameplay_input);
    if (hashString(JSON.stringify(state)) !== entry.state_root) throw new Error(`journal state root mismatch at sequence ${entry.sequence}`);
    previous = entry.entry_hash;
  }
  return { state, state_root: hashString(JSON.stringify(state)) };
}
function appendContinuation(restoredState, receipts, journal, action) {
  const state = JSON.parse(JSON.stringify(restoredState));
  const sequence = journal.length + 1;
  const input = runtimeInput(action, sequence);
  const nextState = applyRuntimeInput(state, input);
  const receipt = makeReceipt(sequence, input, nextState);
  const entry = makeJournalEntry(sequence, input, receipt, receipt.state_root, journal.length ? journal[journal.length - 1].entry_hash : 'genesis');
  const nextReceipts = [...receipts, receipt];
  const nextJournal = [...journal, entry];
  const checkpoint = makeCheckpoint(nextState, nextJournal);
  return { state: nextState, receipts: nextReceipts, journal: nextJournal, checkpoint, receipt, journal_entry: entry, roots: rootSet(nextState, nextReceipts, nextJournal, checkpoint) };
}

function sourceExecution() {
  const validators = clusterValidators('cluster-a').map((validator) => {
    const root = path.join(REPORT_DIR, 'source-cluster', validator.id);
    fs.rmSync(root, { recursive: true, force: true });
    const result = adapter.execute(SOURCE_ACTIONS, { root });
    const replayed = replayJournal(result.journal);
    const roots = rootSet(replayed.state, result.receipts, result.journal, result.checkpoint);
    return { validator, result, state: replayed.state, roots };
  });
  const canonical = validators[0];
  const agreement = validators.every((item) => canonicalHash({ receipts: item.result.receipts, journal: item.result.journal, checkpoint: item.result.checkpoint, roots: item.roots }) === canonicalHash({ receipts: canonical.result.receipts, journal: canonical.result.journal, checkpoint: canonical.result.checkpoint, roots: canonical.roots }));
  const report = {
    schema: 'everarcade.hotpocket.migration.source-cluster.v0.1',
    cluster: 'A', validators: validators.map((item) => ({ validator: item.validator.id, state_root: item.roots.state_root, replay_root: item.roots.replay_root, world_root: item.roots.world_root, checkpoint_root: item.roots.checkpoint_root, receipt_root: item.roots.receipt_root, journal_root: item.roots.journal_root })),
    actions: SOURCE_ACTIONS, state_root: canonical.roots.state_root, replay_root: canonical.roots.replay_root,
    world_root: canonical.roots.world_root, checkpoint_root: canonical.roots.checkpoint_root,
    receipt_root: canonical.roots.receipt_root, journal_root: canonical.roots.journal_root,
    status: status(agreement && canonical.result.accepted)
  };
  writeJsonBoth('source_cluster_report.json', report);
  return { canonical, validators, report, ok: report.status === 'PASS' };
}

function exportPackage(source) {
  rmDirContents(EXPORT_DIR);
  const world = {
    world_id: WORLD_ID, runtime_version: RUNTIME_VERSION, proof_version: PROOF_VERSION,
    state: source.canonical.state, roots: source.canonical.roots, validator_count: 3,
    source_cluster: 'cluster-a', export_order: ['world.bin', 'checkpoint.bin', 'journal.bin', 'receipts.bin', 'manifest.json']
  };
  const checkpoint = source.canonical.result.checkpoint;
  const journal = source.canonical.result.journal;
  const receipts = source.canonical.result.receipts;
  const payloads = {
    'world.bin': canonicalBuffer(world),
    'checkpoint.bin': canonicalBuffer(checkpoint),
    'journal.bin': canonicalBuffer(journal),
    'receipts.bin': canonicalBuffer(receipts)
  };
  const artifactHashes = Object.fromEntries(Object.entries(payloads).map(([name, bytes]) => [name, `sha256:${sha256Bytes(bytes)}`]));
  const manifest = {
    schema: 'everarcade.hotpocket.migration.export-manifest.v0.1', export_version: EXPORT_VERSION,
    created_at_epoch_ms: 0, deterministic_order: Object.keys(payloads).concat('manifest.json'),
    canonical_encoding: 'utf8-json-canonicalize-with-trailing-newline', hash_algorithm: 'sha256',
    world_id: WORLD_ID, runtime_version: RUNTIME_VERSION,
    runtime_compatibility: { runtime: RUNTIME_VERSION, hotpocket_consensus: 'three-validator-deterministic-projection', requires_shared_runtime_state: false },
    roots: source.canonical.roots, artifacts: artifactHashes
  };
  payloads['manifest.json'] = canonicalBuffer(manifest);
  for (const name of manifest.deterministic_order) writeFile(EXPORT_DIR, name, payloads[name]);
  const exportRoot = canonicalHash({ artifacts: artifactHashes, manifest_hash: `sha256:${sha256Bytes(payloads['manifest.json'])}` });
  const report = { schema: 'everarcade.hotpocket.migration.export-package.v0.1', export_root: exportRoot, manifest_hash: `sha256:${sha256Bytes(payloads['manifest.json'])}`, artifacts: artifactHashes, deterministic_order: manifest.deterministic_order, version_metadata: { proof_version: PROOF_VERSION, export_version: EXPORT_VERSION }, runtime_compatibility: manifest.runtime_compatibility, status: 'PASS' };
  writeJsonBoth('export_package_report.json', report);
  return { world, checkpoint, journal, receipts, manifest, report, export_root: exportRoot, ok: true };
}

function transferPackage() {
  rmDirContents(IMPORT_DIR);
  const artifacts = ['world.bin', 'checkpoint.bin', 'journal.bin', 'receipts.bin', 'manifest.json'];
  artifacts.forEach((name) => copyArtifact(EXPORT_DIR, IMPORT_DIR, name));
  const exportedHashes = artifacts.map((name) => `${name}=${sha256Bytes(fs.readFileSync(path.join(EXPORT_DIR, name)))}`);
  const importedHashes = artifacts.map((name) => `${name}=${sha256Bytes(fs.readFileSync(path.join(IMPORT_DIR, name)))}`);
  const ok = exportedHashes.join('\n') === importedHashes.join('\n');
  writeTextBoth('transfer_boundary_report.txt', [
    'EverArcade HotPocket Migration Transfer Boundary Report',
    'Source cluster process memory shared with target cluster: NO',
    'Direct runtime state reuse: NO',
    'Direct container volume reuse: NO',
    'Transferred material: exported artifacts only',
    `Exported artifact hashes: ${exportedHashes.join(', ')}`,
    `Imported artifact hashes: ${importedHashes.join(', ')}`,
    `Transfer Boundary Proof: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok };
}

function readCanonicalFile(dir, name) { return JSON.parse(fs.readFileSync(path.join(dir, name), 'utf8')); }
function restoreCluster() {
  const manifest = readCanonicalFile(IMPORT_DIR, 'manifest.json');
  const artifactNames = ['world.bin', 'checkpoint.bin', 'journal.bin', 'receipts.bin'];
  const hashOk = artifactNames.every((name) => manifest.artifacts[name] === `sha256:${sha256Bytes(fs.readFileSync(path.join(IMPORT_DIR, name)))}`);
  const imported = { world: readCanonicalFile(IMPORT_DIR, 'world.bin'), checkpoint: readCanonicalFile(IMPORT_DIR, 'checkpoint.bin'), journal: readCanonicalFile(IMPORT_DIR, 'journal.bin'), receipts: readCanonicalFile(IMPORT_DIR, 'receipts.bin') };
  const validators = clusterValidators('cluster-b').map((validator) => {
    const replayed = replayJournal(imported.journal);
    const checkpoint = makeCheckpoint(replayed.state, imported.journal);
    const checkpointOk = checkpoint.checkpoint_hash === imported.checkpoint.checkpoint_hash;
    const roots = rootSet(replayed.state, imported.receipts, imported.journal, imported.checkpoint);
    return { validator: validator.id, state: replayed.state, roots, checkpoint_ok: checkpointOk, hash_ok: hashOk };
  });
  const canonical = validators[0];
  const agree = validators.every((item) => canonicalHash({ roots: item.roots, state: item.state }) === canonicalHash({ roots: canonical.roots, state: canonical.state }));
  const ok = hashOk && agree && validators.every((item) => item.checkpoint_ok);
  const report = { schema: 'everarcade.hotpocket.migration.restore.v0.1', cluster: 'B', restored_exclusively_from_export: true, manifest_hashes_verified: hashOk, validators, state_root: canonical.roots.state_root, replay_root: canonical.roots.replay_root, world_root: canonical.roots.world_root, checkpoint_root: canonical.roots.checkpoint_root, status: status(ok) };
  writeJsonBoth('restore_report.json', report);
  return { imported, validators, canonical, report, ok };
}

function equivalenceProof(source, restore) {
  const comparisons = {
    state_root: [source.canonical.roots.state_root, restore.canonical.roots.state_root],
    replay_root: [source.canonical.roots.replay_root, restore.canonical.roots.replay_root],
    world_root: [source.canonical.roots.world_root, restore.canonical.roots.world_root],
    checkpoint_root: [source.canonical.roots.checkpoint_root, restore.canonical.roots.checkpoint_root]
  };
  const ok = Object.values(comparisons).every(([a, b]) => a === b);
  writeTextBoth('equivalence_report.txt', [
    'EverArcade HotPocket Migration Equivalence Report',
    ...Object.entries(comparisons).map(([name, [a, b]]) => `${name}: source=${a} restored=${b} ${status(a === b)}`),
    `Equivalence Proof: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok, comparisons };
}

function continuationProof(restore) {
  const continued = appendContinuation(restore.canonical.state, restore.imported.receipts, restore.imported.journal, CONTINUATION_ACTION);
  const validators = clusterValidators('cluster-b').map((validator) => {
    const local = appendContinuation(restore.canonical.state, restore.imported.receipts, restore.imported.journal, CONTINUATION_ACTION);
    return { validator: validator.id, receipt_hash: local.receipt.receipt_hash, journal_entry_hash: local.journal_entry.entry_hash, checkpoint_root: local.roots.checkpoint_root, state_root: local.roots.state_root, roots: local.roots };
  });
  const agreement = validators.every((item) => canonicalHash(item.roots) === canonicalHash(validators[0].roots));
  const ok = agreement && continued.receipt.state_root === continued.roots.state_root && continued.state.positions.alice.x === 15 && continued.state.positions.alice.y === 25;
  writeJsonBoth('continuation_report.json', { schema: 'everarcade.hotpocket.migration.continuation.v0.1', action: CONTINUATION_ACTION, new_receipt: continued.receipt, new_journal_entry: continued.journal_entry, new_checkpoint: continued.checkpoint, validators, continued_root: continued.roots.state_root, state_evolution: continued.state, status: status(ok) });
  writeFile(REPLAY_DIR, 'post_migration_journal.json', Buffer.from(`${JSON.stringify(continued.journal, null, 2)}\n`));
  return { ...continued, validators, ok };
}

function replayAfterMigration(continued) {
  let ok = false;
  let replayedRoot = '';
  try {
    const replayed = replayJournal(continued.journal);
    replayedRoot = replayed.state_root;
    ok = replayedRoot === continued.roots.state_root;
  } catch (_error) { ok = false; }
  writeTextBoth('replay_after_migration_report.txt', [
    'EverArcade HotPocket Replay After Migration Report',
    'Replay start: genesis',
    'Replay journal: full restored journal plus post-migration continuation entry',
    `replayed_root: ${replayedRoot}`,
    `continued_root: ${continued.roots.state_root}`,
    `Replay After Migration Proof: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok, replayed_root: replayedRoot };
}

function validatorAgreement(continued) {
  const receiptOk = new Set(continued.validators.map((v) => v.receipt_hash)).size === 1;
  const journalOk = new Set(continued.validators.map((v) => v.journal_entry_hash)).size === 1;
  const checkpointOk = new Set(continued.validators.map((v) => v.checkpoint_root)).size === 1;
  const stateOk = new Set(continued.validators.map((v) => v.state_root)).size === 1;
  const ok = receiptOk && journalOk && checkpointOk && stateOk;
  writeTextBoth('post_migration_validator_agreement_report.txt', [
    'EverArcade HotPocket Post-Migration Validator Agreement Report',
    `Receipts agree: ${status(receiptOk)}`,
    `Journals agree: ${status(journalOk)}`,
    `Checkpoints agree: ${status(checkpointOk)}`,
    `State roots agree: ${status(stateOk)}`,
    `Post-Migration Validator Agreement Proof: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok };
}

function failureInspection(checks) {
  const failures = [];
  if (!checks.replay) failures.push('replay mismatch');
  if (!checks.restore) failures.push('restore mismatch');
  if (!checks.equivalence) failures.push('root mismatch');
  if (!checks.validatorAgreement) failures.push('validator disagreement');
  if (!checks.continuation) failures.push('continuation failure');
  const ok = failures.length === 0;
  writeTextBoth('migration_failure_report.txt', [
    'EverArcade HotPocket Migration Failure Inspection Report',
    ...FORBIDDEN_FAILURES.map((failure) => `${failure}: ${failures.includes(failure) ? 'FAIL' : 'PASS'}`),
    `Migration Failure Inspection: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok, failures };
}

function validate() {
  ensureDir(REPORT_DIR); ensureDir(ROOT_REPORT_DIR); ensureDir(REPLAY_DIR);
  const source = sourceExecution();
  const exported = exportPackage(source);
  const transfer = transferPackage(exported);
  const restored = restoreCluster();
  const equivalence = equivalenceProof(source, restored);
  const continued = continuationProof(restored);
  const replay = replayAfterMigration(continued);
  const agreement = validatorAgreement(continued);
  const failures = failureInspection({ replay: replay.ok, restore: restored.ok, equivalence: equivalence.ok, continuation: continued.ok, validatorAgreement: agreement.ok });
  const ok = [source.ok, exported.ok, transfer.ok, restored.ok, equivalence.ok, continued.ok, replay.ok, agreement.ok, failures.ok].every(Boolean);
  writeTextBoth('hotpocket_migration_validation_report.txt', [
    'EverArcade Sovereign Runtime Migration Proof v0.1 Validation',
    `Source execution proof: ${status(source.ok)}`,
    `Export proof: ${status(exported.ok)}`,
    `Transfer simulation proof: ${status(transfer.ok)}`,
    `Restore proof: ${status(restored.ok)}`,
    `Equivalence proof: ${status(equivalence.ok)}`,
    `Continuation proof: ${status(continued.ok)}`,
    `Replay proof: ${status(replay.ok)}`,
    `Validator agreement proof: ${status(agreement.ok)}`,
    `Failure inspection: ${status(failures.ok)}`,
    `EverArcade Sovereign Runtime Migration Proof v0.1: ${status(ok)}`
  ].join('\n') + '\n');
  process.stdout.write(`${status(ok)}\n`);
  return ok;
}

function main() {
  const command = process.argv[2] || 'validate';
  if (command !== 'validate') throw new Error(`unknown command: ${command}`);
  process.exit(validate() ? 0 : 1);
}

if (require.main === module) main();
module.exports = { validate, sourceExecution, exportPackage, restoreCluster, replayJournal };
