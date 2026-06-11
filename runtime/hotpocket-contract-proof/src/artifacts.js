'use strict';

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

const SCHEMA = 'everarcade.hotpocket.execution-proof.v0.1';
const DEFAULT_TIME = '1970-01-01T00:00:00.000Z';

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

function initialState() {
  const state = {
    schema: SCHEMA,
    sequence: 0,
    player_count: 0,
    seen_inputs: [],
    receipts: [],
    journal: [],
    checkpoints: []
  };
  recomputeRoots(state);
  return state;
}

function recomputeRoots(state) {
  state.state_root = rootFor('everarcade-hotpocket-state-v0.1', [
    { id: 'player_count', hash: sha256(state.player_count) },
    { id: 'sequence', hash: sha256(state.sequence) },
    { id: 'seen_inputs', hash: canonicalHash(state.seen_inputs) }
  ]);
  state.receipt_root = rootFor('everarcade-hotpocket-receipts-v0.1', state.receipts.map((r) => ({ id: r.execution_id, hash: r.receipt_hash })));
  state.journal_root = rootFor('everarcade-hotpocket-journal-v0.1', state.journal.map((j) => ({ id: j.execution_id, hash: j.journal_hash })));
  return state;
}

function loadState(stateDir) {
  return readJson(path.join(stateDir, 'state.json'), initialState());
}

function saveState(stateDir, state) {
  recomputeRoots(state);
  writeJson(path.join(stateDir, 'state.json'), state);
  writeJson(path.join(stateDir, 'roots.json'), {
    state_root: state.state_root,
    receipt_root: state.receipt_root,
    journal_root: state.journal_root
  });
}

function normalizeInput(raw) {
  let input = raw;
  if (Buffer.isBuffer(input)) input = input.toString('utf8');
  if (typeof input === 'string') input = JSON.parse(input);
  if (input && typeof input === 'object' && input.payload && !input.action) {
    input = typeof input.payload === 'string' ? JSON.parse(input.payload) : input.payload;
  }
  if (!input || typeof input !== 'object' || Array.isArray(input)) throw new Error('input must be a JSON object');
  if (input.action !== 'ping' && input.action !== 'join_player') throw new Error(`unsupported action: ${input.action}`);
  return input;
}

function executeInput(state, raw, metadata = {}) {
  const input = normalizeInput(raw);
  const input_hash = canonicalHash({ input, metadata: { user: metadata.user || 'hotpocket-user', ledger_seq: metadata.ledger_seq || 0 } });
  if (state.seen_inputs.includes(input_hash)) throw new Error(`duplicate input rejected: ${input_hash}`);

  const before_root = state.state_root;
  const before_player_count = state.player_count;
  if (input.action === 'join_player') state.player_count += 1;
  state.sequence += 1;
  state.seen_inputs.push(input_hash);
  recomputeRoots(state);

  const execution_id = `hp-exec-${String(state.sequence).padStart(6, '0')}`;
  const output = input.action === 'ping' ? { status: 'ok' } : { status: 'ok', player_count: state.player_count };
  const mutation = {
    action: input.action,
    before_player_count,
    after_player_count: state.player_count,
    before_root,
    after_root: state.state_root,
    input_hash,
    sequence: state.sequence
  };
  const journal = {
    schema: 'everarcade.hotpocket.journal.v0.1',
    execution_id,
    sequence: state.sequence,
    input_hash,
    mutation_hash: canonicalHash(mutation),
    before_root,
    after_root: state.state_root,
    deterministic_content_hash: canonicalHash({ input, mutation })
  };
  journal.journal_hash = canonicalHash(journal);
  state.journal.push(journal);

  const receipt = {
    schema: 'everarcade.hotpocket.receipt.v0.1',
    execution_id,
    timestamp: metadata.timestamp || DEFAULT_TIME,
    status: 'ok',
    input_hash,
    state_root: state.state_root,
    output
  };
  receipt.receipt_hash = canonicalHash(receipt);
  state.receipts.push(receipt);
  recomputeRoots(state);
  receipt.receipt_root = state.receipt_root;
  receipt.journal_root = state.journal_root;
  receipt.receipt_hash = canonicalHash(receipt);
  state.receipts[state.receipts.length - 1] = receipt;
  recomputeRoots(state);

  const checkpoint = {
    schema: 'everarcade.hotpocket.checkpoint.v0.1',
    execution_id,
    sequence: state.sequence,
    state_root: state.state_root,
    receipt_root: state.receipt_root,
    journal_root: state.journal_root
  };
  state.checkpoints.push(checkpoint);
  return { output, receipt, journal, checkpoint, state };
}

function runSequence(inputs) {
  const state = initialState();
  const executions = inputs.map((input, index) => executeInput(state, input, { ledger_seq: index + 1, user: `proof-user-${index + 1}` }));
  return {
    executions,
    roots: {
      state_root: state.state_root,
      receipt_root: state.receipt_root,
      journal_root: state.journal_root
    }
  };
}

module.exports = {
  DEFAULT_TIME,
  SCHEMA,
  canonicalHash,
  canonicalize,
  ensureDir,
  executeInput,
  initialState,
  loadState,
  normalizeInput,
  readJson,
  recomputeRoots,
  rootFor,
  runSequence,
  saveState,
  writeJson
};
