#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const adapter = require('../adapter/runtime-adapter');

const ROOT = path.resolve(__dirname, '..');
const STATE_PATH = process.env.EVERARCADE_HOTPOCKET_RUNTIME_CONTRACT_STATE || path.join(ROOT, 'reports', 'contract-runtime-state.json');

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function readState() { try { return JSON.parse(fs.readFileSync(STATE_PATH, 'utf8')); } catch (_error) { return { actions: [] }; } }
function writeState(state) { ensureDir(path.dirname(STATE_PATH)); fs.writeFileSync(STATE_PATH, `${JSON.stringify(state, null, 2)}\n`); }
async function readUserInput(ctx, inputRef) {
  if (!inputRef) return null;
  if (Buffer.isBuffer(inputRef)) return inputRef.toString('utf8');
  if (typeof inputRef === 'string') return inputRef;
  if (typeof inputRef.read === 'function') return (await inputRef.read()).toString('utf8');
  if (inputRef.payload) return Buffer.isBuffer(inputRef.payload) ? inputRef.payload.toString('utf8') : inputRef.payload;
  return JSON.stringify(inputRef);
}
async function handleContext(ctx) {
  const state = readState();
  const outputs = [];
  for (const user of ctx.users.list()) {
    for (const inputRef of user.inputs) {
      const raw = await readUserInput(ctx, inputRef);
      const parsed = typeof raw === 'string' ? JSON.parse(raw) : raw;
      const action = adapter.normalizeActions([parsed])[0];
      state.actions.push(action);
      const execution = adapter.execute(state.actions, { root: path.join(ROOT, 'reports', 'contract-runtime') });
      state.last_state_root = execution.state_root;
      state.last_receipts = execution.receipts;
      state.last_journal = execution.journal;
      state.last_checkpoint = execution.checkpoint;
      const payload = { accepted: true, output: execution.output, receipt: execution.receipts.at(-1), journal: execution.journal.at(-1), checkpoint: execution.checkpoint, state_root: execution.state_root };
      await user.send(payload);
      outputs.push(payload);
    }
  }
  writeState(state);
  return outputs;
}
async function main() {
  const HotPocket = require('hotpocket-nodejs-contract');
  const contract = new HotPocket.Contract();
  const started = await contract.init(handleContext, HotPocket.clientProtocols.json, false);
  if (!started) process.exit(1);
}
module.exports = { handleContext };
if (require.main === module) main().catch((error) => { process.stderr.write(`${error.stack || error.message}\n`); process.exit(1); });
