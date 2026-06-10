#!/usr/bin/env node
import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const ROOT = path.resolve(path.dirname(__filename), '..', '..');
const DATA_DIR = process.env.EVERARCADE_LIVE_DATA_DIR || path.join(ROOT, '.everarcade-live-proof', 'runtime-data');
const HOST = process.env.EVERARCADE_LIVE_HOST || '0.0.0.0';
const PORT = Number(process.env.EVERARCADE_LIVE_PORT || 8787);
const RESET = process.env.EVERARCADE_LIVE_RESET === '1';
const SESSION_ID = process.env.EVERARCADE_LIVE_SESSION || 'arena-live-proof-v0.1';
const TICK_MS = Number(process.env.EVERARCADE_LIVE_TICK_MS || 250);

const dirs = {
  root: DATA_DIR,
  receipts: path.join(DATA_DIR, 'receipts'),
  checkpoints: path.join(DATA_DIR, 'checkpoints'),
  replay: path.join(DATA_DIR, 'replay'),
  status: path.join(DATA_DIR, 'status'),
  logs: path.join(DATA_DIR, 'logs'),
};
const files = {
  state: path.join(DATA_DIR, 'state.json'),
  journal: path.join(DATA_DIR, 'journal.ndjson'),
  replay: path.join(dirs.replay, 'replay-proof.json'),
  status: path.join(dirs.status, 'runtime-status.json'),
  checkpoint: path.join(dirs.checkpoints, 'latest-checkpoint.json'),
  log: path.join(dirs.logs, 'runtime.log'),
};

function ensureDirs() {
  if (RESET && fs.existsSync(DATA_DIR)) fs.rmSync(DATA_DIR, { recursive: true, force: true });
  for (const dir of Object.values(dirs)) fs.mkdirSync(dir, { recursive: true });
}

function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
  if (value && typeof value === 'object') return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(',')}}`;
  return JSON.stringify(value);
}

function hash(value) {
  return crypto.createHash('sha256').update(typeof value === 'string' ? value : canonical(value)).digest('hex');
}

function initialState() {
  return {
    session_id: SESSION_ID,
    template: 'arena',
    wasm_guest: 'everarcade-arena-guest.wasm',
    tick: 0,
    players: {},
    processed_actions: {},
    last_sequence: {},
    rejected_actions: [],
    replay_status: 'recording',
    state_root: '',
  };
}

function computeStateRoot(state) {
  const rootState = { ...state };
  delete rootState.state_root;
  return hash(rootState);
}

function loadState() {
  if (fs.existsSync(files.state)) return JSON.parse(fs.readFileSync(files.state, 'utf8'));
  const state = initialState();
  state.state_root = computeStateRoot(state);
  return state;
}

function writeJson(file, value) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`);
}

function appendJournal(entry) {
  fs.appendFileSync(files.journal, `${JSON.stringify(entry)}\n`);
}

function writeCheckpoint(state, reason = 'periodic') {
  const checkpoint = {
    session_id: state.session_id,
    tick: state.tick,
    state_root: state.state_root,
    reason,
    written_at: new Date().toISOString(),
    state,
  };
  writeJson(files.checkpoint, checkpoint);
  writeJson(path.join(dirs.checkpoints, `checkpoint-${String(state.tick).padStart(6, '0')}.json`), checkpoint);
  return checkpoint;
}

function writeReplay(state) {
  let entries = [];
  if (fs.existsSync(files.journal)) {
    entries = fs.readFileSync(files.journal, 'utf8').trim().split('\n').filter(Boolean).map((line) => JSON.parse(line));
  }
  const replay = {
    proof_version: 'evernode-live-gameplay-proof-v0.1',
    session_id: state.session_id,
    template: state.template,
    action_count: entries.filter((entry) => entry.accepted).length,
    rejected_count: entries.filter((entry) => !entry.accepted).length,
    journal_sha256: hash(entries.map((entry) => canonical(entry)).join('\n')),
    final_tick: state.tick,
    final_state_root: state.state_root,
    replay_status: 'exported',
    exported_at: new Date().toISOString(),
  };
  writeJson(files.replay, replay);
  return replay;
}

function log(message) {
  fs.appendFileSync(files.log, `${new Date().toISOString()} ${message}\n`);
}

function saveState(state) {
  state.state_root = computeStateRoot(state);
  writeJson(files.state, state);
  writeJson(files.status, {
    runtime: 'everarcade-live-arena-runtime',
    status: 'running',
    pid: process.pid,
    port: PORT,
    tick: state.tick,
    state_root: state.state_root,
    updated_at: new Date().toISOString(),
  });
  return state;
}

function receiptFor(entry) {
  const receipt = { ...entry, receipt_sha256: hash(entry) };
  writeJson(path.join(dirs.receipts, `${String(entry.tick).padStart(6, '0')}-${entry.action_id}.json`), receipt);
  return receipt;
}

function reject(state, action, reason) {
  const entry = {
    accepted: false,
    reason,
    tick: state.tick,
    action_id: action.action_id || `rejected-${Date.now()}`,
    player_id: action.player_id || 'unknown',
    action_type: action.action_type || 'unknown',
    sequence: action.sequence ?? null,
    state_root_before: state.state_root,
    state_root_after: state.state_root,
    recorded_at: new Date().toISOString(),
  };
  state.rejected_actions.push(entry);
  appendJournal(entry);
  return receiptFor(entry);
}

function applyAction(state, action) {
  const normalized = {
    action_id: String(action.action_id || `${action.player_id || 'anon'}-${action.sequence ?? Date.now()}`),
    player_id: String(action.player_id || ''),
    action_type: String(action.action_type || ''),
    sequence: Number(action.sequence),
    direction: action.direction ? String(action.direction) : undefined,
  };

  if (!normalized.player_id || !normalized.action_type || !Number.isInteger(normalized.sequence)) {
    return reject(state, normalized, 'invalid-schema');
  }
  if (state.processed_actions[normalized.action_id]) {
    return reject(state, normalized, 'duplicate-action');
  }
  const lastSequence = state.last_sequence[normalized.player_id] || 0;
  if (normalized.sequence !== lastSequence + 1) {
    return reject(state, normalized, 'out-of-order-sequence');
  }

  const before = state.state_root;
  if (normalized.action_type === 'join') {
    state.players[normalized.player_id] = state.players[normalized.player_id] || { id: normalized.player_id, x: 0, y: 0, health: 100, score: 0 };
  } else {
    if (!state.players[normalized.player_id]) return reject(state, normalized, 'unknown-player');
    const player = state.players[normalized.player_id];
    if (normalized.action_type === 'move') {
      if (normalized.direction === 'north') player.y += 1;
      else if (normalized.direction === 'south') player.y -= 1;
      else if (normalized.direction === 'east') player.x += 1;
      else if (normalized.direction === 'west') player.x -= 1;
      else return reject(state, normalized, 'invalid-direction');
      player.score += 1;
    } else if (normalized.action_type === 'attack') {
      const targetId = Object.keys(state.players).find((id) => id !== normalized.player_id);
      if (!targetId) return reject(state, normalized, 'missing-target');
      state.players[targetId].health = Math.max(0, state.players[targetId].health - 10);
      player.score += 5;
    } else if (normalized.action_type === 'score') {
      player.score += Number(action.points || 1);
    } else {
      return reject(state, normalized, 'invalid-action');
    }
  }

  state.tick += 1;
  state.last_sequence[normalized.player_id] = normalized.sequence;
  state.processed_actions[normalized.action_id] = true;
  state.replay_status = 'recording';
  saveState(state);
  const entry = {
    accepted: true,
    tick: state.tick,
    action_id: normalized.action_id,
    player_id: normalized.player_id,
    action_type: normalized.action_type,
    sequence: normalized.sequence,
    state_root_before: before,
    state_root_after: state.state_root,
    recorded_at: new Date().toISOString(),
  };
  appendJournal(entry);
  const receipt = receiptFor(entry);
  if (state.tick % 2 === 0) writeCheckpoint(state, 'action-boundary');
  writeReplay(state);
  return receipt;
}

async function readBody(req) {
  const chunks = [];
  for await (const chunk of req) chunks.push(chunk);
  const body = Buffer.concat(chunks).toString('utf8');
  return body ? JSON.parse(body) : {};
}

function send(res, status, payload) {
  const body = JSON.stringify(payload, null, 2);
  res.writeHead(status, {
    'content-type': 'application/json',
    'access-control-allow-origin': '*',
    'access-control-allow-methods': 'GET,POST,OPTIONS',
    'access-control-allow-headers': 'content-type',
  });
  res.end(`${body}\n`);
}

ensureDirs();
let state = loadState();
saveState(state);
writeCheckpoint(state, 'runtime-start');
writeReplay(state);
log(`runtime-start pid=${process.pid} port=${PORT}`);
const tickTimer = setInterval(() => {
  writeReplay(state);
}, TICK_MS).unref();

const server = http.createServer(async (req, res) => {
  try {
    const url = new URL(req.url, `http://${req.headers.host || 'localhost'}`);
    if (req.method === 'OPTIONS') return send(res, 200, { ok: true });
    if (req.method === 'GET' && url.pathname === '/health') return send(res, 200, { ok: true, status: 'running', pid: process.pid, tick: state.tick, state_root: state.state_root });
    if (req.method === 'GET' && url.pathname === '/state') return send(res, 200, state);
    if (req.method === 'GET' && url.pathname === '/replay') return send(res, 200, fs.existsSync(files.replay) ? JSON.parse(fs.readFileSync(files.replay, 'utf8')) : writeReplay(state));
    if (req.method === 'POST' && url.pathname === '/action') {
      const action = await readBody(req);
      const receipt = applyAction(state, action);
      const accepted = Boolean(receipt.accepted);
      return send(res, accepted ? 200 : 409, receipt);
    }
    if (req.method === 'POST' && url.pathname === '/checkpoint') return send(res, 200, writeCheckpoint(state, 'client-request'));
    return send(res, 404, { error: 'not-found' });
  } catch (error) {
    return send(res, 500, { error: String(error.message || error) });
  }
});

server.listen(PORT, HOST, () => log(`http-listen host=${HOST} port=${PORT}`));

function shutdown(signal) {
  log(`runtime-stop signal=${signal}`);
  writeCheckpoint(state, 'runtime-stop');
  writeReplay(state);
  clearInterval(tickTimer);
  server.close(() => process.exit(0));
  setTimeout(() => process.exit(0), 1000).unref();
}
process.on('SIGTERM', () => shutdown('SIGTERM'));
process.on('SIGINT', () => shutdown('SIGINT'));
