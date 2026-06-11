'use strict';

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

const ADAPTER_VERSION = '0.1.0';
const CANONICAL_ACTIONS = new Set(['ping', 'join_player']);

function sha256(value) {
  return crypto.createHash('sha256').update(String(value)).digest('hex');
}

function canonicalize(value) {
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonicalize).join(',')}]`;
  return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalize(value[key])}`).join(',')}}`;
}

function canonicalHash(value) {
  return sha256(canonicalize(value));
}

function rootFor(label, records) {
  const lines = [label];
  for (const record of [...records].sort((a, b) => a.id.localeCompare(b.id))) {
    lines.push(`${record.id}:${record.hash}`);
  }
  return sha256(lines.join('\n'));
}

function ensureDir(dir) {
  fs.mkdirSync(dir, { recursive: true });
}

function readJson(file, fallback) {
  if (!fs.existsSync(file)) return fallback;
  return JSON.parse(fs.readFileSync(file, 'utf8'));
}

function writeJson(file, value) {
  ensureDir(path.dirname(file));
  fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`);
}

function createInitialState() {
  return {
    adapter_version: ADAPTER_VERSION,
    sequence: 0,
    player_count: 0,
    seen_input_hashes: [],
    receipts: [],
    journal: [],
    checkpoints: [],
    replay_proofs: [],
    state_root: rootFor('everarcade-state-v0.1', [{ id: 'genesis', hash: sha256('0') }]),
    receipt_root: rootFor('everarcade-receipts-v0.1', []),
    journal_root: rootFor('everarcade-journal-v0.1', []),
    replay_root: rootFor('everarcade-replay-v0.1', [])
  };
}

function loadState(stateDir) {
  return readJson(path.join(stateDir, 'state.json'), createInitialState());
}

function saveState(stateDir, state) {
  writeJson(path.join(stateDir, 'state.json'), state);
  writeJson(path.join(stateDir, 'roots.json'), {
    state_root: state.state_root,
    receipt_root: state.receipt_root,
    journal_root: state.journal_root,
    replay_root: state.replay_root
  });
}

function normalizeHotPocketInput(input, fallbackUser = 'hotpocket-user') {
  let payload = input;
  if (Buffer.isBuffer(input)) payload = input.toString('utf8');
  if (typeof payload === 'string') payload = JSON.parse(payload);
  if (payload && typeof payload === 'object' && payload.payload && !payload.action) {
    payload = typeof payload.payload === 'string' ? JSON.parse(payload.payload) : payload.payload;
  }
  if (!payload || typeof payload !== 'object' || Array.isArray(payload)) {
    throw new Error('HotPocket input must be a JSON object');
  }
  const action = payload.action;
  if (!CANONICAL_ACTIONS.has(action)) throw new Error(`Unsupported action: ${action}`);
  return {
    action,
    player_id: payload.player_id || fallbackUser,
    nonce: payload.nonce || `${action}:${payload.player_id || fallbackUser}`,
    payload
  };
}

function toRuntimeInput(hotpocketInput, metadata = {}) {
  const normalized = normalizeHotPocketInput(hotpocketInput, metadata.user || metadata.user_id || 'hotpocket-user');
  return {
    schema: 'everarcade.runtime.input.v0.1',
    source: 'hotpocket',
    action: normalized.action,
    player_id: normalized.player_id,
    nonce: normalized.nonce,
    payload: normalized.payload,
    metadata: {
      ledger_seq: metadata.ledger_seq || metadata.ledger || 0,
      user: metadata.user || metadata.user_id || normalized.player_id,
      received_at: metadata.received_at || 'deterministic-proof-time'
    }
  };
}

function recomputeRoots(state) {
  state.state_root = rootFor('everarcade-state-v0.1', [
    { id: 'player_count', hash: sha256(state.player_count) },
    { id: 'sequence', hash: sha256(state.sequence) },
    { id: 'seen_input_hashes', hash: canonicalHash(state.seen_input_hashes) }
  ]);
  state.receipt_root = rootFor('everarcade-receipts-v0.1', state.receipts.map((receipt) => ({ id: receipt.receipt_id, hash: receipt.receipt_hash })));
  state.journal_root = rootFor('everarcade-journal-v0.1', state.journal.map((entry) => ({ id: entry.journal_id, hash: entry.journal_hash })));
  state.replay_root = rootFor('everarcade-replay-v0.1', state.replay_proofs.map((proof) => ({ id: proof.replay_id, hash: proof.replay_hash })));
}

function executeRuntimeInput(state, runtimeInput) {
  const inputHash = canonicalHash(runtimeInput);
  if (state.seen_input_hashes.includes(inputHash)) throw new Error(`Replay rejected: ${inputHash}`);

  const before = { player_count: state.player_count, state_root: state.state_root };
  if (runtimeInput.action === 'join_player') state.player_count += 1;
  if (runtimeInput.action === 'ping') {
    // Ping is an execution proof and intentionally leaves game state unchanged.
  }

  state.sequence += 1;
  state.seen_input_hashes.push(inputHash);
  const mutation = {
    action: runtimeInput.action,
    input_hash: inputHash,
    before,
    after: { player_count: state.player_count },
    sequence: state.sequence
  };
  const mutationHash = canonicalHash(mutation);
  const receipt = {
    schema: 'everarcade.receipt.v0.1',
    receipt_id: `receipt-${String(state.sequence).padStart(6, '0')}`,
    status: 'ok',
    action: runtimeInput.action,
    input_hash: inputHash,
    mutation_hash: mutationHash,
    output: runtimeInput.action === 'ping' ? { status: 'ok' } : { status: 'ok', player_count: state.player_count }
  };
  receipt.receipt_hash = canonicalHash(receipt);
  const journalEntry = {
    schema: 'everarcade.journal.v0.1',
    journal_id: `journal-${String(state.sequence).padStart(6, '0')}`,
    input_hash: inputHash,
    mutation_hash: mutationHash,
    before_root: before.state_root,
    player_count_before: before.player_count,
    player_count_after: state.player_count
  };
  journalEntry.journal_hash = canonicalHash(journalEntry);
  const checkpoint = {
    schema: 'everarcade.checkpoint.v0.1',
    checkpoint_id: `checkpoint-${String(state.sequence).padStart(6, '0')}`,
    sequence: state.sequence,
    player_count: state.player_count,
    seen_input_hashes: [...state.seen_input_hashes]
  };
  checkpoint.checkpoint_hash = canonicalHash(checkpoint);
  const replayProof = {
    schema: 'everarcade.replay-proof.v0.1',
    replay_id: `replay-${String(state.sequence).padStart(6, '0')}`,
    input_hash: inputHash,
    mutation_hash: mutationHash,
    checkpoint_hash: checkpoint.checkpoint_hash
  };
  replayProof.replay_hash = canonicalHash(replayProof);

  state.receipts.push(receipt);
  state.journal.push(journalEntry);
  state.checkpoints.push(checkpoint);
  state.replay_proofs.push(replayProof);
  recomputeRoots(state);
  receipt.state_root = state.state_root;
  receipt.receipt_root = state.receipt_root;
  receipt.journal_root = state.journal_root;
  receipt.replay_root = state.replay_root;
  return { receipt, journalEntry, checkpoint, replayProof, state };
}

function executeHotPocketInput(input, options = {}) {
  const stateDir = options.stateDir || path.resolve('runtime/hotpocket-adapter/.state');
  const state = options.state || loadState(stateDir);
  const runtimeInput = toRuntimeInput(input, options.metadata || {});
  const result = executeRuntimeInput(state, runtimeInput);
  if (!options.state) saveState(stateDir, state);
  return { runtimeInput, ...result };
}

function runSequence(inputs) {
  const state = createInitialState();
  const outputs = [];
  for (let i = 0; i < inputs.length; i += 1) {
    outputs.push(executeHotPocketInput(inputs[i], { state, metadata: { ledger_seq: i + 1, user: `user-${i + 1}` } }));
  }
  return { state, outputs, roots: {
    state_root: state.state_root,
    receipt_root: state.receipt_root,
    journal_root: state.journal_root,
    replay_root: state.replay_root
  } };
}

module.exports = {
  ADAPTER_VERSION,
  canonicalize,
  canonicalHash,
  createInitialState,
  executeHotPocketInput,
  executeRuntimeInput,
  loadState,
  normalizeHotPocketInput,
  rootFor,
  runSequence,
  saveState,
  toRuntimeInput
};
