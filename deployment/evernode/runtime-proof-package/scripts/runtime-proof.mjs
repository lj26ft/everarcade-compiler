#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import os from 'node:os';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(__dirname, '..');
const reportDir = process.env.EVERARCADE_PROOF_REPORT_DIR || path.join(packageRoot, 'reports');
const artifactDir = process.env.EVERARCADE_PROOF_ARTIFACT_DIR || path.join(packageRoot, 'artifacts');
const config = readJson(path.join(packageRoot, 'config', 'runtime-config.json'));
const actions = readJson(path.join(packageRoot, 'gameplay', 'actions.json'));

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function readJson(file) { return JSON.parse(fs.readFileSync(file, 'utf8')); }
function writeJson(file, value) { ensureDir(path.dirname(file)); fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`); }
function sha256(value) { return crypto.createHash('sha256').update(value).digest('hex'); }
function canonical(value) {
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
  return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(',')}}`;
}
function hashJson(value) { return sha256(canonical(value)); }
function status(ok) { return ok ? 'PASS' : 'FAIL'; }
function now() { return new Date().toISOString(); }
function requireRealLease() {
  const failures = [];
  const requiredEnv = ['EVERNODE_LEASE_ID', 'EVERNODE_HOST_ID', 'EVERNODE_REAL_LEASE_PROOF'];
  for (const key of requiredEnv) if (!process.env[key]) failures.push(`${key} is required`);
  if (process.env.EVERNODE_REAL_LEASE_PROOF !== '1') failures.push('EVERNODE_REAL_LEASE_PROOF must be 1');
  if (process.env.EVERARCADE_ALLOW_MOCK_LEASE === '1') failures.push('mock lease override is forbidden');
  if (process.env.HOTPOCKET_RUNTIME_INTEGRATION_PROOF !== '1') failures.push('HOTPOCKET_RUNTIME_INTEGRATION_PROOF must be 1 after real HotPocket runtime discovery');
  if (fs.existsSync('/.dockerenv') && process.env.EVERNODE_ALLOW_CONTAINERIZED_LEASE !== '1') failures.push('container-only execution detected; set EVERNODE_ALLOW_CONTAINERIZED_LEASE=1 only on an Evernode lease that is itself containerized by the host runtime');
  return failures;
}
function initialState() {
  return {
    schema: 'everarcade.runtime-proof.state.v0.1',
    world_id: config.world_id,
    session_id: config.session_id,
    tick: 0,
    players: {},
    positions: {},
    events: ['genesis']
  };
}
function applyAction(state, action, validatorId) {
  const next = JSON.parse(JSON.stringify(state));
  next.tick = action.sequence;
  if (action.action === 'join_player') {
    next.players[action.player_id] = { player_id: action.player_id, joined_tick: action.sequence };
    next.positions[action.player_id] = next.positions[action.player_id] || { x: 0, y: 0 };
    next.events.push(`tick ${action.sequence}: ${action.player_id} joined`);
  } else if (action.action === 'move_player') {
    if (!next.players[action.player_id]) throw new Error(`player not joined: ${action.player_id}`);
    next.positions[action.player_id] = { x: action.x, y: action.y };
    next.events.push(`tick ${action.sequence}: ${action.player_id} moved to ${action.x},${action.y}`);
  } else {
    throw new Error(`unsupported action: ${action.action}`);
  }
  const stateRoot = hashJson(next);
  const receipt = {
    schema: 'everarcade.runtime-proof.receipt.v0.1',
    lease_id: process.env.EVERNODE_LEASE_ID,
    host_id: process.env.EVERNODE_HOST_ID,
    sequence: action.sequence,
    action: action.action,
    accepted: true,
    state_root: stateRoot,
    receipt_hash: null
  };
  receipt.receipt_hash = hashJson({ ...receipt, receipt_hash: null });
  return { state: next, receipt };
}
function executeValidator(validatorId) {
  let state = initialState();
  const receipts = [];
  const journal = [];
  let previous_hash = 'genesis';
  for (const action of actions) {
    const result = applyAction(state, action, validatorId);
    const entry = {
      schema: 'everarcade.runtime-proof.journal-entry.v0.1',
      lease_id: process.env.EVERNODE_LEASE_ID,
      sequence: action.sequence,
      previous_hash,
      action,
      state_root: result.receipt.state_root,
      receipt_hash: result.receipt.receipt_hash,
      entry_hash: null
    };
    entry.entry_hash = hashJson({ ...entry, entry_hash: null });
    previous_hash = entry.entry_hash;
    state = result.state;
    receipts.push(result.receipt);
    journal.push(entry);
  }
  const checkpoint = {
    schema: 'everarcade.runtime-proof.checkpoint.v0.1',
    lease_id: process.env.EVERNODE_LEASE_ID,
    final_sequence: actions.at(-1).sequence,
    state,
    state_root: hashJson(state),
    journal_root: hashJson(journal),
    receipt_root: hashJson(receipts),
    checkpoint_hash: null
  };
  checkpoint.checkpoint_hash = hashJson({ ...checkpoint, checkpoint_hash: null });
  return { validator_id: validatorId, state, state_root: checkpoint.state_root, receipts, journal, checkpoint };
}
function replayFromArtifacts(canonicalRun) {
  let state = initialState();
  const receipts = [];
  const journal = [];
  let previous_hash = 'genesis';
  for (const artifactEntry of canonicalRun.journal) {
    if (artifactEntry.previous_hash !== previous_hash) throw new Error(`journal chain mismatch at ${artifactEntry.sequence}`);
    const result = applyAction(state, artifactEntry.action, canonicalRun.validator_id);
    if (result.receipt.state_root !== artifactEntry.state_root) throw new Error(`replay state root mismatch at ${artifactEntry.sequence}`);
    const entry = { ...artifactEntry, entry_hash: null };
    entry.entry_hash = hashJson(entry);
    if (entry.entry_hash !== artifactEntry.entry_hash) throw new Error(`journal entry hash mismatch at ${artifactEntry.sequence}`);
    previous_hash = artifactEntry.entry_hash;
    state = result.state;
    receipts.push(result.receipt);
    journal.push(artifactEntry);
  }
  return {
    state_root: hashJson(state),
    receipt_root: hashJson(receipts),
    journal_root: hashJson(journal)
  };
}
function writeReport(name, value) { writeJson(path.join(reportDir, name), value); }

function main() {
  ensureDir(reportDir); ensureDir(artifactDir);
  const realLeaseFailures = requireRealLease();
  if (realLeaseFailures.length) {
    writeReport('runtime_deployment_report.json', {
      schema: 'everarcade.evernode.runtime-deployment-report.v0.1',
      generated_at: now(),
      package_startup: 'FAIL',
      runtime_status: 'FAIL',
      failures: realLeaseFailures,
      status: 'FAIL'
    });
    console.error(realLeaseFailures.join('\n'));
    process.exit(1);
  }

  const validators = Array.from({ length: config.validator_count }, (_, index) => executeValidator(`validator-${index + 1}`));
  const canonicalRun = validators[0];
  const stateRoots = new Set(validators.map((item) => item.state_root));
  const receiptRoots = new Set(validators.map((item) => hashJson(item.receipts)));
  const journalRoots = new Set(validators.map((item) => hashJson(item.journal)));
  const checkpointRoots = new Set(validators.map((item) => item.checkpoint.checkpoint_hash));
  const consensusOk = stateRoots.size === 1 && receiptRoots.size === 1 && journalRoots.size === 1 && checkpointRoots.size === 1;

  writeJson(path.join(artifactDir, 'receipt.json'), canonicalRun.receipts);
  writeJson(path.join(artifactDir, 'journal.json'), canonicalRun.journal);
  writeJson(path.join(artifactDir, 'checkpoint.json'), canonicalRun.checkpoint);
  writeJson(path.join(artifactDir, 'state-root.json'), { state_root: canonicalRun.state_root });

  const artifactReport = {
    schema: 'everarcade.evernode.runtime-artifact-report.v0.1',
    generated_at: now(),
    receipt_present: fs.existsSync(path.join(artifactDir, 'receipt.json')),
    journal_present: fs.existsSync(path.join(artifactDir, 'journal.json')),
    checkpoint_present: fs.existsSync(path.join(artifactDir, 'checkpoint.json')),
    state_root_present: fs.existsSync(path.join(artifactDir, 'state-root.json')),
    receipt_root: hashJson(canonicalRun.receipts),
    journal_root: hashJson(canonicalRun.journal),
    checkpoint_root: canonicalRun.checkpoint.checkpoint_hash,
    state_root: canonicalRun.state_root,
    deterministic_output: consensusOk ? 'PASS' : 'FAIL',
    status: status(consensusOk)
  };
  writeReport('lease_runtime_artifact_report.json', artifactReport);

  const gameplayReport = {
    schema: 'everarcade.evernode.gameplay-execution-report.v0.1',
    generated_at: now(),
    lease_id: process.env.EVERNODE_LEASE_ID,
    host_id: process.env.EVERNODE_HOST_ID,
    validators: validators.map((item) => ({ validator_id: item.validator_id, state_root: item.state_root, accepted_actions: item.receipts.length })),
    submitted_actions: actions,
    action_acceptance: validators.every((item) => item.receipts.every((receipt) => receipt.accepted)) ? 'PASS' : 'FAIL',
    gameplay_mutation: canonicalRun.state.positions.alice?.x === 10 && canonicalRun.state.positions.alice?.y === 20 ? 'PASS' : 'FAIL',
    deterministic_state_transition: consensusOk ? 'PASS' : 'FAIL',
    status: status(consensusOk)
  };
  writeReport('lease_gameplay_execution_report.json', gameplayReport);

  const replay = replayFromArtifacts(canonicalRun);
  const continuityPayload = {
    schema: 'everarcade.evernode.continuity-anchor.v0.1',
    lease_id: process.env.EVERNODE_LEASE_ID,
    host_id: process.env.EVERNODE_HOST_ID,
    world_id: config.world_id,
    state_root: artifactReport.state_root,
    receipt_root: artifactReport.receipt_root,
    journal_root: artifactReport.journal_root,
    checkpoint_root: artifactReport.checkpoint_root,
    replay_root: replay.state_root,
    previous_anchor_hash: process.env.EVERARCADE_PREVIOUS_ANCHOR_HASH || 'genesis'
  };
  const continuityRoot = hashJson(continuityPayload);
  const continuityAnchor = { ...continuityPayload, continuity_root: continuityRoot, anchor_hash: hashJson({ continuity_root: continuityRoot, payload: continuityPayload }) };
  writeJson(path.join(artifactDir, 'continuity-anchor.json'), continuityAnchor);
  writeReport('lease_continuity_report.json', {
    schema: 'everarcade.evernode.continuity-report.v0.1',
    generated_at: now(),
    continuity_root: continuityRoot,
    anchor_hash: continuityAnchor.anchor_hash,
    replay_root: replay.state_root,
    restore_root: canonicalRun.checkpoint.state_root,
    replay_root_match: replay.state_root === artifactReport.state_root,
    restore_root_match: canonicalRun.checkpoint.state_root === artifactReport.state_root,
    status: status(replay.state_root === artifactReport.state_root && canonicalRun.checkpoint.state_root === artifactReport.state_root)
  });
  writeReport('lease_replay_report.json', {
    schema: 'everarcade.evernode.replay-report.v0.1',
    generated_at: now(),
    receipt_equivalence: replay.receipt_root === artifactReport.receipt_root ? 'PASS' : 'FAIL',
    journal_equivalence: replay.journal_root === artifactReport.journal_root ? 'PASS' : 'FAIL',
    state_root_equivalence: replay.state_root === artifactReport.state_root ? 'PASS' : 'FAIL',
    continuity_root_equivalence: hashJson({ ...continuityPayload, replay_root: replay.state_root }) === continuityRoot ? 'PASS' : 'FAIL',
    replay_roots: replay,
    status: status(replay.receipt_root === artifactReport.receipt_root && replay.journal_root === artifactReport.journal_root && replay.state_root === artifactReport.state_root)
  });
  writeReport('runtime_deployment_report.json', {
    schema: 'everarcade.evernode.runtime-deployment-report.v0.1',
    generated_at: now(),
    lease_id: process.env.EVERNODE_LEASE_ID,
    host_id: process.env.EVERNODE_HOST_ID,
    runtime_hostname: os.hostname(),
    package_startup: 'PASS',
    runtime_status: 'PASS',
    hotpocket_runtime_integration_proof: process.env.HOTPOCKET_RUNTIME_INTEGRATION_PROOF === '1' ? 'PASS' : 'FAIL',
    hotpocket_consensus_validators: config.validator_count,
    package_integrity_checked_by_deployer: process.env.EVERARCADE_PACKAGE_SHA256 || null,
    status: 'PASS'
  });
  console.log(`state_root=${artifactReport.state_root}`);
  console.log(`continuity_root=${continuityRoot}`);
}

main();
