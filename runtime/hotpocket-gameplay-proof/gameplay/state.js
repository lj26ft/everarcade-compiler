'use strict';

const crypto = require('crypto');

const DEFAULT_TIME = '1970-01-01T00:00:00.000Z';
const SCHEMA = 'everarcade.hotpocket.consensus-gameplay-proof.v0.1';

function canonicalize(value) {
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonicalize).join(',')}]`;
  return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalize(value[key])}`).join(',')}}`;
}

function sha256(value) {
  return crypto.createHash('sha256').update(String(value)).digest('hex');
}

function canonicalHash(value) {
  return sha256(canonicalize(value));
}

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function genesisState() {
  return { players: {}, tick: 0 };
}

function validateAction(input) {
  if (!input || typeof input !== 'object' || Array.isArray(input)) throw new Error('gameplay action must be a JSON object');
  if (input.action === 'ping') return { action: 'ping' };
  if (input.action === 'join_player') {
    if (typeof input.player_id !== 'string' || input.player_id.length === 0) throw new Error('join_player requires player_id');
    return { action: 'join_player', player_id: input.player_id };
  }
  if (input.action === 'move_player') {
    if (typeof input.player_id !== 'string' || input.player_id.length === 0) throw new Error('move_player requires player_id');
    if (!Number.isInteger(input.x) || !Number.isInteger(input.y)) throw new Error('move_player requires integer x and y');
    return { action: 'move_player', player_id: input.player_id, x: input.x, y: input.y };
  }
  throw new Error(`unsupported gameplay action: ${input.action}`);
}

function applyAction(state, input) {
  const action = validateAction(input);
  const before = clone(state);
  const after = clone(state);
  let mutation = 'none';
  if (action.action === 'join_player') {
    if (!after.players[action.player_id]) {
      after.players[action.player_id] = { x: 0, y: 0 };
      mutation = 'player_joined';
    } else {
      mutation = 'player_already_joined';
    }
  } else if (action.action === 'move_player') {
    if (!after.players[action.player_id]) throw new Error(`cannot move missing player: ${action.player_id}`);
    after.players[action.player_id] = { x: action.x, y: action.y };
    mutation = 'player_moved';
  } else if (action.action === 'ping') {
    mutation = 'ping_acknowledged';
  }
  after.tick += 1;
  return { action, before, after, mutation };
}

function execute(state, input, sequence, metadata = {}) {
  const { action, before, after, mutation } = applyAction(state, input);
  const canonical_input = canonicalize(action);
  const canonical_state_before = canonicalize(before);
  const canonical_state_after = canonicalize(after);
  const state_before_hash = sha256(canonical_state_before);
  const state_after_hash = sha256(canonical_state_after);
  const action_hash = sha256(canonical_input);
  const execution_id = `gameplay-exec-${String(sequence).padStart(6, '0')}`;
  const state_root = state_after_hash;
  const output = { accepted: true, action: action.action, mutation, state_root, tick: after.tick, players: clone(after.players) };
  const receiptBase = {
    schema: `${SCHEMA}.receipt`,
    execution_id,
    sequence,
    status: 'accepted',
    generated_at: DEFAULT_TIME,
    validator_independent: true,
    action_hash,
    state_before_hash,
    state_after_hash,
    state_root,
    output
  };
  const receipt = { ...receiptBase, receipt_hash: canonicalHash(receiptBase) };
  const journalBase = {
    schema: `${SCHEMA}.journal-entry`,
    execution_id,
    sequence,
    canonical_input,
    canonical_state_before,
    canonical_state_after,
    action,
    state_before: before,
    state_after: after,
    state_before_hash,
    state_after_hash,
    action_hash,
    receipt_hash: receipt.receipt_hash,
    state_root,
    mutation
  };
  const journal = { ...journalBase, journal_hash: canonicalHash(journalBase) };
  return { state: after, canonical_input, canonical_state_before, canonical_state_after, state_root, state_before_hash, state_after_hash, action_hash, receipt, journal, output };
}

function replay(actions) {
  let state = genesisState();
  const executions = [];
  actions.forEach((action, index) => {
    const result = execute(state, action, index + 1, { replay: true });
    executions.push(result);
    state = result.state;
  });
  return { state, state_root: sha256(canonicalize(state)), executions };
}

module.exports = { DEFAULT_TIME, SCHEMA, canonicalize, sha256, canonicalHash, genesisState, validateAction, execute, replay };
