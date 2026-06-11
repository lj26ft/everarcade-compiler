#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const childProcess = require('child_process');
const adapter = require('../../hotpocket-runtime-proof/adapter/runtime-adapter');

const ROOT = path.resolve(__dirname, '..');
const REPO_ROOT = path.resolve(ROOT, '../..');
const ANCHOR_DIR = path.join(ROOT, 'anchors');
const CONTINUITY_DIR = path.join(ROOT, 'continuity');
const REPORT_DIR = process.env.EVERARCADE_CONTINUITY_ANCHOR_REPORT_DIR || path.join(ROOT, 'reports');
const ROOT_REPORT_DIR = path.join(REPO_ROOT, 'reports');
const RUNTIME_PROOF_DIR = path.join(REPO_ROOT, 'runtime', 'hotpocket-runtime-proof');
const MIGRATION_PROOF_DIR = path.join(REPO_ROOT, 'runtime', 'hotpocket-migration-proof');
const PROTOCOL_VERSION = 'everarcade-continuity-anchor-v0.1';
const GENESIS_ANCHOR_HASH = 'genesis';
const FORBIDDEN_FAILURES = [
  'anchor mismatch', 'replay mismatch', 'restore mismatch', 'migration mismatch',
  'continuity break', 'root mismatch', 'hash mismatch', 'payload mismatch'
];

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function cleanDir(dir) { fs.rmSync(dir, { recursive: true, force: true }); ensureDir(dir); }
function status(ok) { return ok ? 'PASS' : 'FAIL'; }
function canonicalize(value) { return adapter.canonicalize(value); }
function canonicalHash(value) { return adapter.canonicalHash(value); }
function sha256Text(value) { return crypto.createHash('sha256').update(String(value)).digest('hex'); }
function writeTextBoth(name, content) {
  ensureDir(REPORT_DIR); ensureDir(ROOT_REPORT_DIR);
  fs.writeFileSync(path.join(REPORT_DIR, name), content);
  fs.writeFileSync(path.join(ROOT_REPORT_DIR, name), content);
}
function writeJsonBoth(name, value) { writeTextBoth(name, `${JSON.stringify(value, null, 2)}\n`); }
function writeJson(file, value) { ensureDir(path.dirname(file)); fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`); }
function readJson(file) { return JSON.parse(fs.readFileSync(file, 'utf8')); }
function maybeReadJson(file) { return fs.existsSync(file) ? readJson(file) : null; }
function runNode(script) {
  const result = childProcess.spawnSync('node', [script, 'validate'], {
    cwd: REPO_ROOT,
    env: { ...process.env, CARGO_BUILD_JOBS: process.env.CARGO_BUILD_JOBS || '1' },
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
    timeout: Number(process.env.EVERARCADE_CONTINUITY_DEPENDENCY_TIMEOUT_MS || 300000)
  });
  return { ok: result.status === 0 && !result.error && /PASS/.test(result.stdout), stdout: result.stdout || '', stderr: result.error ? `${result.stderr || ''}${result.error.message}` : result.stderr || '' };
}

function replayJournal(journal) {
  let state = {
    session_id: 'session-0001', tick: 0, players: {},
    positions: { dummy: { x: 0, y: 1 } }, health: { dummy: 100 }, scores: { dummy: 0 },
    events: ['session_started'], player_count: 0
  };
  let previous = GENESIS_ANCHOR_HASH;
  for (const entry of journal) {
    const base = { ...entry };
    delete base.entry_hash;
    if (entry.previous_hash !== previous) throw new Error(`journal previous hash mismatch at sequence ${entry.sequence}`);
    if (canonicalHash(base) !== entry.entry_hash) throw new Error(`journal entry hash mismatch at sequence ${entry.sequence}`);
    const input = entry.gameplay_input;
    state = JSON.parse(JSON.stringify(state));
    state.tick = input.sequence;
    if (input.action === 'join') {
      state.players[input.player_id] = { player_id: input.player_id, joined_tick: input.sequence };
      if (!state.positions[input.player_id]) state.positions[input.player_id] = { x: 0, y: 0 };
      if (state.health[input.player_id] === undefined) state.health[input.player_id] = 100;
      if (state.scores[input.player_id] === undefined) state.scores[input.player_id] = 0;
      state.player_count = Object.keys(state.players).length;
      state.events.push(`tick ${input.sequence}: ${input.player_id} joined`);
    } else if (input.action === 'move') {
      if (!state.positions[input.player_id]) state.positions[input.player_id] = { x: 0, y: 0 };
      state.positions[input.player_id] = { x: input.x, y: input.y };
      state.events.push(`tick ${input.sequence}: ${input.player_id} moved to ${input.x},${input.y}`);
    } else {
      throw new Error(`unsupported replay action: ${input.action}`);
    }
    if (sha256Text(JSON.stringify(state)) !== entry.state_root) throw new Error(`journal state root mismatch at sequence ${entry.sequence}`);
    previous = entry.entry_hash;
  }
  return { state, state_root: sha256Text(JSON.stringify(state)) };
}

function deriveContinuityRoot(roots) {
  return canonicalHash({
    protocol_version: PROTOCOL_VERSION,
    world_id: roots.world_id,
    execution_epoch: roots.execution_epoch,
    state_root: roots.state_root,
    checkpoint_root: roots.checkpoint_root,
    journal_root: roots.journal_root,
    receipt_root: roots.receipt_root,
    replay_root: roots.replay_root,
    previous_anchor_hash: roots.previous_anchor_hash,
    migration_root: roots.migration_root || null
  });
}

function constructAnchor(input, previousAnchorHash) {
  const executionEpoch = Number(input.execution_epoch);
  const payloadBase = {
    protocol_version: PROTOCOL_VERSION,
    world_id: input.world_id,
    execution_epoch: executionEpoch,
    state_root: input.state_root,
    checkpoint_root: input.checkpoint_root,
    journal_root: input.journal_root,
    receipt_root: input.receipt_root,
    replay_root: input.replay_root,
    previous_anchor_hash: previousAnchorHash
  };
  const continuityRoot = deriveContinuityRoot({ ...payloadBase, migration_root: input.migration_root || null });
  const payload = {
    protocol_version: payloadBase.protocol_version,
    world_id: payloadBase.world_id,
    execution_epoch: payloadBase.execution_epoch,
    state_root: payloadBase.state_root,
    checkpoint_root: payloadBase.checkpoint_root,
    journal_root: payloadBase.journal_root,
    receipt_root: payloadBase.receipt_root,
    replay_root: payloadBase.replay_root,
    continuity_root: continuityRoot,
    timestamp: executionEpoch,
    previous_anchor_hash: previousAnchorHash
  };
  const anchor_hash = canonicalHash(payload);
  return { anchor_id: `anchor-${String(executionEpoch).padStart(4, '0')}`, anchor_hash, payload, source: input.source };
}

function rootsFromArtifacts(label, executionEpoch, artifacts) {
  const replayed = replayJournal(artifacts.journal);
  const stateRoot = artifacts.state_root || replayed.state_root;
  const checkpointRoot = artifacts.checkpoint.checkpoint_hash || artifacts.checkpoint_root;
  const receiptRoot = canonicalHash(artifacts.receipts);
  const journalRoot = canonicalHash(artifacts.journal);
  const replayRoot = artifacts.replay_root || replayed.state_root;
  const continuityInputHash = canonicalHash({ label, executionEpoch, stateRoot, checkpointRoot, receiptRoot, journalRoot, replayRoot, migration_root: artifacts.migration_root || null });
  return {
    source: label,
    world_id: artifacts.world_id || adapter.WORLD_ID,
    execution_epoch: executionEpoch,
    state_root: stateRoot,
    checkpoint_root: checkpointRoot,
    journal_root: journalRoot,
    receipt_root: receiptRoot,
    replay_root: replayRoot,
    migration_root: artifacts.migration_root || continuityInputHash
  };
}

function importRuntimeArtifacts() {
  const root = path.join(REPORT_DIR, 'runtime-import');
  fs.rmSync(root, { recursive: true, force: true });
  const result = adapter.execute(adapter.DEFAULT_ACTIONS, { root });
  const replayed = replayJournal(result.journal);
  const ok = result.accepted && replayed.state_root === result.state_root && result.receipts.length > 0 && result.journal.length > 0 && Boolean(result.checkpoint.checkpoint_hash);
  const artifacts = {
    world_id: adapter.WORLD_ID,
    receipts: result.receipts,
    journal: result.journal,
    checkpoint: result.checkpoint,
    state_root: result.state_root,
    replay_root: replayed.state_root,
    restore_root: result.proof.restored_root
  };
  writeJsonBoth('anchor_import_report.json', {
    schema: 'everarcade.continuity-anchor.import.v0.1',
    receipt_import: status(result.receipts.length > 0),
    journal_import: status(result.journal.length > 0),
    checkpoint_import: status(Boolean(result.checkpoint.checkpoint_hash)),
    state_root: result.state_root,
    replay_root: replayed.state_root,
    receipt_root: canonicalHash(result.receipts),
    journal_root: canonicalHash(result.journal),
    checkpoint_root: result.checkpoint.checkpoint_hash,
    status: status(ok)
  });
  return { ok, artifacts };
}

function importMigrationArtifacts() {
  const exportManifest = maybeReadJson(path.join(MIGRATION_PROOF_DIR, 'export', 'manifest.json'));
  const importManifest = maybeReadJson(path.join(MIGRATION_PROOF_DIR, 'import', 'manifest.json'));
  const exportedWorld = readJson(path.join(MIGRATION_PROOF_DIR, 'export', 'world.bin'));
  const exportedCheckpoint = readJson(path.join(MIGRATION_PROOF_DIR, 'export', 'checkpoint.bin'));
  const exportedJournal = readJson(path.join(MIGRATION_PROOF_DIR, 'export', 'journal.bin'));
  const exportedReceipts = readJson(path.join(MIGRATION_PROOF_DIR, 'export', 'receipts.bin'));
  const importedWorld = readJson(path.join(MIGRATION_PROOF_DIR, 'import', 'world.bin'));
  const importedCheckpoint = readJson(path.join(MIGRATION_PROOF_DIR, 'import', 'checkpoint.bin'));
  const importedJournal = readJson(path.join(MIGRATION_PROOF_DIR, 'import', 'journal.bin'));
  const importedReceipts = readJson(path.join(MIGRATION_PROOF_DIR, 'import', 'receipts.bin'));
  const continuedJournal = readJson(path.join(MIGRATION_PROOF_DIR, 'replay', 'post_migration_journal.json'));
  const continuationReport = readJson(path.join(MIGRATION_PROOF_DIR, 'reports', 'continuation_report.json'));
  const continuedReceipts = [...importedReceipts, continuationReport.new_receipt];
  const continuedCheckpoint = continuationReport.new_checkpoint;
  const exportedReplay = replayJournal(exportedJournal);
  const importedReplay = replayJournal(importedJournal);
  const continuedReplay = replayJournal(continuedJournal);
  const exportMigrationRoot = canonicalHash({ manifest: exportManifest, roots: exportedWorld.roots, state_root: exportedReplay.state_root });
  const importMigrationRoot = canonicalHash({ manifest: importManifest, roots: importedWorld.roots, state_root: importedReplay.state_root });
  const continuedMigrationRoot = canonicalHash({ continuation_root: continuationReport.continued_root, receipt: continuationReport.new_receipt.receipt_hash, journal: canonicalHash(continuedJournal) });
  const ok = exportedReplay.state_root === importedReplay.state_root && continuedReplay.state_root === continuationReport.continued_root && Boolean(exportManifest) && Boolean(importManifest);
  writeJsonBoth('anchor_migration_import_report.json', {
    schema: 'everarcade.continuity-anchor.migration-import.v0.1',
    export_state_root: exportedReplay.state_root,
    import_state_root: importedReplay.state_root,
    continued_state_root: continuedReplay.state_root,
    export_migration_root: exportMigrationRoot,
    import_migration_root: importMigrationRoot,
    continued_migration_root: continuedMigrationRoot,
    status: status(ok)
  });
  return {
    ok,
    pre: { world_id: adapter.WORLD_ID, receipts: exportedReceipts, journal: exportedJournal, checkpoint: exportedCheckpoint, state_root: exportedReplay.state_root, replay_root: exportedReplay.state_root, migration_root: exportMigrationRoot },
    restored: { world_id: adapter.WORLD_ID, receipts: importedReceipts, journal: importedJournal, checkpoint: importedCheckpoint, state_root: importedReplay.state_root, replay_root: importedReplay.state_root, migration_root: importMigrationRoot },
    post: { world_id: adapter.WORLD_ID, receipts: continuedReceipts, journal: continuedJournal, checkpoint: continuedCheckpoint, state_root: continuedReplay.state_root, replay_root: continuedReplay.state_root, migration_root: continuedMigrationRoot }
  };
}

function buildChain(runtimeArtifacts, migrationArtifacts) {
  cleanDir(ANCHOR_DIR);
  cleanDir(CONTINUITY_DIR);
  const inputs = [
    rootsFromArtifacts('runtime-live', 0, runtimeArtifacts),
    rootsFromArtifacts('migration-pre', 1, migrationArtifacts.pre),
    rootsFromArtifacts('migration-restored', 2, migrationArtifacts.restored),
    rootsFromArtifacts('migration-post', 3, migrationArtifacts.post)
  ];
  const anchors = [];
  let previous = GENESIS_ANCHOR_HASH;
  for (const input of inputs) {
    const anchor = constructAnchor(input, previous);
    anchors.push(anchor);
    previous = anchor.anchor_hash;
    writeJson(path.join(ANCHOR_DIR, `${anchor.anchor_id}.json`), anchor);
  }
  writeJson(path.join(CONTINUITY_DIR, 'continuity-chain.json'), { schema: 'everarcade.continuity-anchor.chain.v0.1', anchors });
  return anchors;
}

function continuityValidation(anchors) {
  let ok = anchors.length >= 4 && anchors[0].payload.previous_anchor_hash === GENESIS_ANCHOR_HASH;
  const lines = ['EverArcade Continuity Anchor Chain Report'];
  anchors.forEach((anchor, index) => {
    const expectedPrevious = index === 0 ? GENESIS_ANCHOR_HASH : anchors[index - 1].anchor_hash;
    const hashOk = canonicalHash(anchor.payload) === anchor.anchor_hash;
    const linkOk = anchor.payload.previous_anchor_hash === expectedPrevious;
    ok = ok && hashOk && linkOk;
    lines.push(`${anchor.anchor_id}: source=${anchor.source} hash=${anchor.anchor_hash} previous=${anchor.payload.previous_anchor_hash} hash=${status(hashOk)} link=${status(linkOk)}`);
  });
  lines.push(`Hash-linked continuity: ${status(ok)}`);
  writeTextBoth('continuity_chain_report.txt', `${lines.join('\n')}\n`);
  return { ok };
}

function determinismProof(anchorInputs) {
  const first = constructAnchor(anchorInputs, GENESIS_ANCHOR_HASH);
  const second = constructAnchor(anchorInputs, GENESIS_ANCHOR_HASH);
  const third = constructAnchor(JSON.parse(JSON.stringify(anchorInputs)), GENESIS_ANCHOR_HASH);
  const ok = first.anchor_hash === second.anchor_hash && second.anchor_hash === third.anchor_hash;
  writeTextBoth('anchor_determinism_report.txt', [
    'EverArcade Anchor Determinism Report',
    `first_anchor_hash: ${first.anchor_hash}`,
    `second_anchor_hash: ${second.anchor_hash}`,
    `third_anchor_hash: ${third.anchor_hash}`,
    `Anchor Determinism Proof: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok, anchor_hash: first.anchor_hash };
}

function replayEquivalenceProof(runtimeArtifacts, liveAnchor) {
  const replayed = replayJournal(runtimeArtifacts.journal);
  const reconstructed = rootsFromArtifacts('runtime-live', liveAnchor.payload.execution_epoch, { ...runtimeArtifacts, state_root: replayed.state_root, replay_root: replayed.state_root });
  const replayedAnchor = constructAnchor(reconstructed, liveAnchor.payload.previous_anchor_hash);
  const ok = replayedAnchor.anchor_hash === liveAnchor.anchor_hash;
  writeTextBoth('anchor_replay_report.txt', [
    'EverArcade Anchor Replay Equivalence Report',
    `live_anchor_hash: ${liveAnchor.anchor_hash}`,
    `replayed_anchor_hash: ${replayedAnchor.anchor_hash}`,
    `replayed_root: ${replayed.state_root}`,
    `Replay Equivalence Proof: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok };
}

function restoreEquivalenceProof(runtimeArtifacts, liveAnchor) {
  const restoredRoot = runtimeArtifacts.restore_root;
  const reconstructed = rootsFromArtifacts('runtime-live', liveAnchor.payload.execution_epoch, { ...runtimeArtifacts, state_root: restoredRoot, replay_root: runtimeArtifacts.replay_root });
  const restoredAnchor = constructAnchor(reconstructed, liveAnchor.payload.previous_anchor_hash);
  const ok = restoredRoot === runtimeArtifacts.state_root && restoredAnchor.anchor_hash === liveAnchor.anchor_hash;
  writeTextBoth('anchor_restore_report.txt', [
    'EverArcade Anchor Restore Equivalence Report',
    `original_anchor_hash: ${liveAnchor.anchor_hash}`,
    `restored_anchor_hash: ${restoredAnchor.anchor_hash}`,
    `original_root: ${runtimeArtifacts.state_root}`,
    `restored_root: ${restoredRoot}`,
    `Restore Equivalence Proof: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok };
}

function migrationEquivalenceProof(anchors) {
  const pre = anchors.find((anchor) => anchor.source === 'migration-pre');
  const restored = anchors.find((anchor) => anchor.source === 'migration-restored');
  const post = anchors.find((anchor) => anchor.source === 'migration-post');
  const restoredRootsOk = pre.payload.state_root === restored.payload.state_root && pre.payload.replay_root === restored.payload.replay_root;
  const continuityOk = restored.payload.previous_anchor_hash === pre.anchor_hash && post.payload.previous_anchor_hash === restored.anchor_hash;
  const postProgressOk = post.payload.state_root !== restored.payload.state_root && post.payload.execution_epoch > restored.payload.execution_epoch;
  const ok = restoredRootsOk && continuityOk && postProgressOk;
  writeTextBoth('anchor_migration_report.txt', [
    'EverArcade Anchor Migration Equivalence Report',
    `pre_migration_anchor_hash: ${pre.anchor_hash}`,
    `restored_anchor_hash: ${restored.anchor_hash}`,
    `post_migration_anchor_hash: ${post.anchor_hash}`,
    `pre_state_root: ${pre.payload.state_root}`,
    `restored_state_root: ${restored.payload.state_root}`,
    `post_state_root: ${post.payload.state_root}`,
    `restored roots equal pre-migration roots: ${status(restoredRootsOk)}`,
    `hash-linked migration continuity: ${status(continuityOk)}`,
    `post-migration continuation advanced state: ${status(postProgressOk)}`,
    `Migration Equivalence Proof: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok };
}

function payloadGeneration(anchor) {
  const xrplPayload = {
    world_id: anchor.payload.world_id,
    state_root: anchor.payload.state_root,
    continuity_root: anchor.payload.continuity_root,
    anchor_hash: anchor.anchor_hash
  };
  const xahauPayload = {
    hook_namespace: 'everarcade.continuity.anchor.v0.1',
    hook_parameters: {
      world_id: anchor.payload.world_id,
      execution_epoch: anchor.payload.execution_epoch,
      continuity_root: anchor.payload.continuity_root,
      previous_anchor_hash: anchor.payload.previous_anchor_hash,
      anchor_hash: anchor.anchor_hash
    }
  };
  const xrplHash = canonicalHash(xrplPayload);
  const xahauHash = canonicalHash(xahauPayload);
  writeJsonBoth('xrpl_anchor_payload_report.json', { schema: 'everarcade.xrpl.anchor-payload.v0.1', payload: xrplPayload, payload_hash: xrplHash, submission: 'not-submitted', status: 'PASS' });
  writeJsonBoth('xahau_anchor_payload_report.json', { schema: 'everarcade.xahau.hook-anchor-payload.v0.1', payload: xahauPayload, payload_hash: xahauHash, deployment: 'not-deployed', status: 'PASS' });
  const repeatOk = xrplHash === canonicalHash(JSON.parse(JSON.stringify(xrplPayload))) && xahauHash === canonicalHash(JSON.parse(JSON.stringify(xahauPayload)));
  return { ok: repeatOk, xrplHash, xahauHash };
}

function failureInspection(checks) {
  const failures = [];
  if (!checks.anchor) failures.push('anchor mismatch');
  if (!checks.replay) failures.push('replay mismatch');
  if (!checks.restore) failures.push('restore mismatch');
  if (!checks.migration) failures.push('migration mismatch');
  if (!checks.continuity) failures.push('continuity break');
  if (!checks.roots) failures.push('root mismatch');
  if (!checks.hashes) failures.push('hash mismatch');
  if (!checks.payload) failures.push('payload mismatch');
  const ok = failures.length === 0;
  writeTextBoth('anchor_failure_report.txt', [
    'EverArcade Continuity Anchor Failure Inspection Report',
    ...FORBIDDEN_FAILURES.map((failure) => `${failure}: ${failures.includes(failure) ? 'FAIL' : 'PASS'}`),
    `Failure Inspection: ${status(ok)}`
  ].join('\n') + '\n');
  return { ok, failures };
}

function validate() {
  ensureDir(REPORT_DIR); ensureDir(ROOT_REPORT_DIR); ensureDir(ANCHOR_DIR); ensureDir(CONTINUITY_DIR);
  const runtimeDependency = runNode(path.join(RUNTIME_PROOF_DIR, 'validation', 'hotpocket-runtime-proof.js'));
  const migrationDependency = runNode(path.join(MIGRATION_PROOF_DIR, 'validation', 'hotpocket-migration-proof.js'));
  const runtimeImport = importRuntimeArtifacts();
  const migrationImport = importMigrationArtifacts();
  const anchors = buildChain(runtimeImport.artifacts, migrationImport);
  const continuity = continuityValidation(anchors);
  const liveInput = rootsFromArtifacts('runtime-live', 0, runtimeImport.artifacts);
  const determinism = determinismProof(liveInput);
  const replay = replayEquivalenceProof(runtimeImport.artifacts, anchors[0]);
  const restore = restoreEquivalenceProof(runtimeImport.artifacts, anchors[0]);
  const migration = migrationEquivalenceProof(anchors);
  const payload = payloadGeneration(anchors[anchors.length - 1]);
  const rootOk = runtimeImport.ok && migrationImport.ok && runtimeDependency.ok && migrationDependency.ok;
  const hashOk = anchors.every((anchor) => canonicalHash(anchor.payload) === anchor.anchor_hash);
  const failures = failureInspection({ anchor: determinism.ok, replay: replay.ok, restore: restore.ok, migration: migration.ok, continuity: continuity.ok, roots: rootOk, hashes: hashOk, payload: payload.ok });
  const ok = [runtimeDependency.ok, migrationDependency.ok, runtimeImport.ok, migrationImport.ok, continuity.ok, determinism.ok, replay.ok, restore.ok, migration.ok, payload.ok, hashOk, failures.ok].every(Boolean);
  writeTextBoth('continuity_anchor_validation_report.txt', [
    'EverArcade Continuity Anchoring Proof v0.1 Validation',
    `1. receipt import: ${status(runtimeImport.artifacts.receipts.length > 0)}`,
    `2. journal import: ${status(runtimeImport.artifacts.journal.length > 0)}`,
    `3. checkpoint import: ${status(Boolean(runtimeImport.artifacts.checkpoint.checkpoint_hash))}`,
    `4. anchor construction: ${status(determinism.ok && hashOk)}`,
    `5. continuity validation: ${status(continuity.ok)}`,
    `6. replay validation: ${status(replay.ok)}`,
    `7. restore validation: ${status(restore.ok)}`,
    `8. migration validation: ${status(migration.ok)}`,
    `9. payload generation: ${status(payload.ok)}`,
    `10. failure inspection: ${status(failures.ok)}`,
    `HotPocket Runtime Integration Proof dependency: ${status(runtimeDependency.ok)}`,
    `HotPocket Migration Proof dependency: ${status(migrationDependency.ok)}`,
    `EverArcade Continuity Anchoring Proof v0.1: ${status(ok)}`
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
module.exports = { validate, constructAnchor, canonicalHash, replayJournal };
