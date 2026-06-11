#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const gameplay = require('../gameplay/state');

const ROOT = path.resolve(__dirname, '..');
const STATE_FILE = process.env.HOTPOCKET_GAMEPLAY_STATE_FILE || path.join(ROOT, 'reports', 'contract_state.json');
const REPORT_DIR = process.env.EVERARCADE_HOTPOCKET_GAMEPLAY_REPORT_DIR || path.join(ROOT, 'reports');

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function readState() {
  if (!fs.existsSync(STATE_FILE)) return { state: gameplay.genesisState(), sequence: 0, journal: [], receipts: [] };
  return JSON.parse(fs.readFileSync(STATE_FILE, 'utf8'));
}
function writeState(record) {
  ensureDir(path.dirname(STATE_FILE));
  fs.writeFileSync(STATE_FILE, `${JSON.stringify(record, null, 2)}\n`);
}
function writeLastExecution(outputs, record) {
  ensureDir(REPORT_DIR);
  fs.writeFileSync(path.join(REPORT_DIR, 'hotpocket_gameplay_last_execution.json'), `${JSON.stringify({ schema: 'everarcade.hotpocket.gameplay.live-execution.v0.1', outputs, state_root: gameplay.sha256(gameplay.canonicalize(record.state)), receipts: record.receipts, journal: record.journal }, null, 2)}\n`);
}
async function readUserInput(ctx, inputRef) {
  const raw = await ctx.users.read(inputRef);
  return raw.toString('utf8');
}
async function handleContext(ctx) {
  const record = readState();
  const outputs = [];
  for (const user of ctx.users.list()) {
    for (const inputRef of user.inputs) {
      const raw = await readUserInput(ctx, inputRef);
      const parsed = typeof raw === 'string' ? JSON.parse(raw) : raw;
      const execution = gameplay.execute(record.state, parsed, record.sequence + 1, { user: user.publicKey || 'hotpocket-user' });
      record.state = execution.state;
      record.sequence += 1;
      record.receipts.push(execution.receipt);
      record.journal.push(execution.journal);
      const payload = { accepted: true, output: execution.output, receipt: execution.receipt, journal: execution.journal };
      await user.send(payload);
      outputs.push(payload);
    }
  }
  writeState(record);
  writeLastExecution(outputs, record);
  return outputs;
}
async function main() {
  const HotPocket = require('hotpocket-nodejs-contract');
  const contract = new HotPocket.Contract();
  const started = await contract.init(handleContext, HotPocket.clientProtocols.json, false);
  if (!started) process.exit(1);
}
module.exports = { handleContext };
if (require.main === module) main().catch((error) => {
  process.stderr.write(`${error.stack || error.message}\n`);
  process.exit(1);
});
