import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';

export const SCHEMA = 'everarcade.hotpocket.arena-wrapper.v0.1';
export const DIRECTIONS = Object.freeze({ north: [0, -1], south: [0, 1], east: [1, 0], west: [-1, 0] });
export const GENESIS = Object.freeze({ tick: 0, players: {}, combat_events: [], last_sequence: {}, commitments: [] });

export function canonicalize(value) {
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonicalize).join(',')}]`;
  return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalize(value[key])}`).join(',')}}`;
}
export function sha256(value) { return createHash('sha256').update(String(value)).digest('hex'); }
export function canonicalHash(value) { return sha256(canonicalize(value)); }
export function clone(value) { return JSON.parse(JSON.stringify(value)); }
export function genesisState() { return clone(GENESIS); }

export function validateEnvelope(input) {
  if (!input || typeof input !== 'object' || Array.isArray(input)) throw new Error('input must be a JSON object');
  const { action, player } = input;
  if (!['join', 'move', 'attack', 'disconnect'].includes(action)) throw new Error(`unsupported action: ${action}`);
  if (typeof player !== 'string' || player.length === 0) throw new Error(`${action} requires player`);
  if (action === 'move') {
    if (!Object.prototype.hasOwnProperty.call(DIRECTIONS, input.direction)) throw new Error('move requires direction north|south|east|west');
    return { action, direction: input.direction, player };
  }
  if (action === 'attack') {
    if (typeof input.target !== 'string' || input.target.length === 0) throw new Error('attack requires target');
    return { action, player, target: input.target };
  }
  return { action, player };
}

function ensurePlayer(state, player) {
  if (!state.players[player]) state.players[player] = { id: player, x: 0, y: 0, health: 100, connected: false, score: 0 };
  return state.players[player];
}

export function applyArenaInput(state, envelope, tickOverride) {
  const action = validateEnvelope(envelope);
  const before = clone(state);
  const after = clone(state);
  let mutation = 'noop';
  const player = ensurePlayer(after, action.player);
  if (action.action === 'join') { player.connected = true; mutation = before.players[action.player] ? 'player_rejoined' : 'player_joined'; }
  if (action.action === 'disconnect') { player.connected = false; mutation = 'player_disconnected'; }
  if (action.action === 'move') {
    if (!player.connected) throw new Error(`cannot move disconnected player: ${action.player}`);
    const [dx, dy] = DIRECTIONS[action.direction];
    player.x += dx; player.y += dy; mutation = 'player_moved';
  }
  if (action.action === 'attack') {
    if (!player.connected) throw new Error(`cannot attack from disconnected player: ${action.player}`);
    const target = ensurePlayer(after, action.target);
    target.health = Math.max(0, target.health - 25);
    player.score += 10;
    const event = { tick: tickOverride ?? after.tick + 1, attacker: action.player, target: action.target, damage: 25, target_health: target.health };
    after.combat_events.push(event);
    mutation = 'player_attacked';
  }
  after.tick = tickOverride ?? after.tick + 1;
  after.last_sequence[action.player] = after.tick;
  return { action, before, after, mutation };
}

export function commitFor(state, receipts) {
  const world = { tick: state.tick, players: state.players, combat_events: state.combat_events };
  const world_hash = canonicalHash(world);
  const receipt_root = canonicalHash(receipts.map((receipt) => receipt.receipt_hash));
  const state_root = canonicalHash(state);
  const continuity_root = canonicalHash({ state_root, receipt_root, world_hash, tick: state.tick });
  return { tick: state.tick, state_root, receipt_root, world_hash, continuity_root };
}

export function executeInput(state, envelope, sequence, priorReceipts = [], tickOverride) {
  const { action, before, after, mutation } = applyArenaInput(state, envelope, tickOverride);
  const canonical_input = canonicalize(action);
  const state_before_hash = canonicalHash(before);
  const action_hash = sha256(canonical_input);
  const execution_id = `arena-hotpocket-${String(sequence).padStart(6, '0')}`;
  const baseReceipt = { schema: `${SCHEMA}.receipt`, execution_id, sequence, round: after.tick, status: 'accepted', generated_at: '1970-01-01T00:00:00.000Z', action_hash, state_before_hash, mutation };
  const tempReceipt = { ...baseReceipt, state_root: canonicalHash(after) };
  const receipt = { ...tempReceipt, receipt_hash: canonicalHash(tempReceipt) };
  const commitments = commitFor(after, [...priorReceipts, receipt]);
  after.commitments.push(commitments);
  const output = { accepted: true, action: action.action, mutation, tick: after.tick, players: clone(after.players), combat_events: clone(after.combat_events), ...commitments };
  Object.assign(receipt, { output, state_root: commitments.state_root, receipt_root: commitments.receipt_root, world_hash: commitments.world_hash, continuity_root: commitments.continuity_root });
  receipt.receipt_hash = canonicalHash({ ...receipt, receipt_hash: undefined });
  const journalBase = { schema: `${SCHEMA}.journal-entry`, execution_id, sequence, round: after.tick, canonical_input, canonical_state_before: canonicalize(before), canonical_state_after: canonicalize(after), action, state_before: before, state_after: after, action_hash, receipt_hash: receipt.receipt_hash, mutation, ...commitments };
  const journal = { ...journalBase, journal_hash: canonicalHash(journalBase) };
  return { state: after, receipt, journal, output, commitments };
}

export function replayJournal(journal) {
  let state = genesisState();
  const receipts = [];
  const outputs = [];
  for (const [index, entry] of journal.entries()) {
    const result = executeInput(state, entry.action, index + 1, receipts, entry.round);
    receipts.push(result.receipt); outputs.push(result.output); state = result.state;
  }
  return { state, receipts, outputs, commitments: state.commitments.at(-1) || commitFor(state, receipts) };
}

export class ArenaHotPocketRuntime {
  constructor({ statePath, journalPath } = {}) { this.statePath = statePath; this.journalPath = journalPath; this.state = genesisState(); this.receipts = []; this.journal = []; }
  load() { if (this.statePath && existsSync(this.statePath)) Object.assign(this, JSON.parse(readFileSync(this.statePath, 'utf8'))); return this; }
  persist() { const snapshot = { state: this.state, receipts: this.receipts, journal: this.journal }; for (const file of [this.statePath, this.journalPath].filter(Boolean)) { mkdirSync(dirname(file), { recursive: true }); } if (this.statePath) writeFileSync(this.statePath, `${JSON.stringify(snapshot, null, 2)}\n`); if (this.journalPath) writeFileSync(this.journalPath, `${JSON.stringify(this.journal, null, 2)}\n`); }
  process(input) { return this.processAtRound(input, this.journal.length + 1); }
  processAtRound(input, round) { const result = executeInput(this.state, input, this.journal.length + 1, this.receipts, round); this.state = result.state; this.receipts.push(result.receipt); this.journal.push(result.journal); this.persist(); return result; }
  processRound(inputs, round) { const results = []; for (const input of inputs) results.push(this.processAtRound(input, round)); return results; }
  snapshot() { const latest = this.state.commitments.at(-1) || commitFor(this.state, this.receipts); return { ...clone(this.state), receipts: this.receipts, journal_size: this.journal.length, replay_status: this.verify().ok ? 'verified' : 'mismatch', ...latest }; }
  verify() { const replayed = replayJournal(this.journal); const live = this.state.commitments.at(-1) || commitFor(this.state, this.receipts); const ok = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'].every((key) => replayed.commitments[key] === live[key]); return { ok, live, replayed: replayed.commitments } }
}

export function defaultPaths(root) {
  if (typeof root !== 'string' || root.length === 0) throw new Error('defaultPaths requires an explicit runtime root path');
  return { statePath: join(root, 'evernode/hotpocket/arena-wrapper-state.json'), journalPath: join(root, 'evernode/journals/arena-hotpocket-journal.json') };
}
export function inputId(envelope) { return `arena-${canonicalHash(envelope)}`; }
