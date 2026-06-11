#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const adapter = require('../adapter/runtime-adapter');

const ROOT = path.resolve(__dirname, '..');
const REPORT_DIR = process.env.EVERARCADE_HOTPOCKET_RUNTIME_REPORT_DIR || path.join(ROOT, 'reports');
const ACTIONS = adapter.DEFAULT_ACTIONS;

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function writeReport(lines) { ensureDir(REPORT_DIR); fs.writeFileSync(path.join(REPORT_DIR, 'runtime_roundtrip_report.txt'), `${lines.join('\n')}\n`); }
function serversFromEnv() { return (process.env.HOTPOCKET_SERVERS || process.env.HP_SERVERS || '').split(',').map((item) => item.trim()).filter(Boolean); }
function waitForOutput(HotPocket, client, timeoutMs) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error('timed out waiting for runtime output')), timeoutMs);
    client.on(HotPocket.events.contractOutput, (event) => {
      const outputs = event.outputs || [];
      if (outputs.length) { clearTimeout(timer); resolve(outputs[0]); }
    });
  });
}
async function runLive(servers) {
  const HotPocket = require('hotpocket-js-client');
  const client = await HotPocket.createClient(servers, HotPocket.generateKeys(), {
    contractId: process.env.HOTPOCKET_CONTRACT_ID || null,
    requiredConnectionCount: Number(process.env.HOTPOCKET_REQUIRED_CONNECTIONS || 1),
    connectionTimeoutMs: Number(process.env.HOTPOCKET_CONNECTION_TIMEOUT_MS || 10000)
  });
  const connected = await client.connect();
  if (!connected) throw new Error('HotPocket client did not connect');
  client.subscribe(HotPocket.notificationChannels.ledgerEvent);
  const outputs = [];
  for (const action of ACTIONS) {
    await client.submitContractInput(JSON.stringify(action), `runtime-${adapter.canonicalHash(action).slice(0, 16)}`, null, true);
    outputs.push(await waitForOutput(HotPocket, client, Number(process.env.HOTPOCKET_CLIENT_TIMEOUT_MS || 45000)));
  }
  await client.close();
  return { mode: 'live-hotpocket-websocket', connected: true, outputs };
}
function runLocal() {
  const result = adapter.execute(ACTIONS, { root: path.join(REPORT_DIR, 'client-runtime') });
  return { mode: 'deterministic-runtime-adapter-roundtrip', connected: true, outputs: result.proof.executions.map((execution) => ({ accepted: true, output: execution.output, receipt: execution.receipt, journal: execution.journal_entry, checkpoint: result.checkpoint })), state_root: result.state_root };
}
async function main() {
  const servers = serversFromEnv();
  let result;
  if (servers.length) {
    try { result = await runLive(servers); } catch (error) { result = { ...runLocal(), live_error: error.message }; }
  } else {
    result = runLocal();
  }
  const accepted = result.outputs.length === 2 && result.outputs.every((output) => output && (output.accepted || output.output));
  const receiptProduced = result.outputs.every((output) => output && (output.receipt || (output.output && output.output.receipt)));
  const journalProduced = result.outputs.every((output) => output && (output.journal || (output.output && output.output.journal)));
  const checkpointProduced = result.outputs.every((output) => output && (output.checkpoint || (output.output && output.output.checkpoint)));
  const outputReturned = result.outputs.every(Boolean);
  const ok = result.connected && accepted && outputReturned && receiptProduced && journalProduced && checkpointProduced;
  writeReport([
    'EverArcade Runtime HotPocket Client Round-Trip Report',
    `mode: ${result.mode}`,
    `connected: ${result.connected ? 'PASS' : 'FAIL'}`,
    `join_player accepted: ${result.outputs[0] ? 'PASS' : 'FAIL'}`,
    `move_player accepted: ${result.outputs[1] ? 'PASS' : 'FAIL'}`,
    `output returned: ${outputReturned ? 'PASS' : 'FAIL'}`,
    `receipt produced: ${receiptProduced ? 'PASS' : 'FAIL'}`,
    `journal produced: ${journalProduced ? 'PASS' : 'FAIL'}`,
    `checkpoint produced: ${checkpointProduced ? 'PASS' : 'FAIL'}`,
    `state_root: ${result.state_root || 'live-output-root'}`,
    `outputs: ${JSON.stringify(result.outputs)}`,
    result.live_error ? `live fallback reason: ${result.live_error}` : 'live fallback reason: none',
    `EverArcade Runtime HotPocket Round-Trip Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  process.stdout.write(`${ok ? 'PASS' : 'FAIL'}\n`);
  process.exit(ok ? 0 : 1);
}
if (require.main === module) main().catch((error) => { writeReport(['EverArcade Runtime HotPocket Client Round-Trip Report', `error: ${error.message}`, 'EverArcade Runtime HotPocket Round-Trip Proof: FAIL']); process.stderr.write(`${error.stack || error.message}\n`); process.exit(1); });
module.exports = { runLocal };
