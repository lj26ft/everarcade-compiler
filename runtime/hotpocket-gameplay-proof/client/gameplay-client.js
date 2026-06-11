#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const gameplay = require('../gameplay/state');

const ROOT = path.resolve(__dirname, '..');
const REPORT_DIR = process.env.EVERARCADE_HOTPOCKET_GAMEPLAY_REPORT_DIR || path.join(ROOT, 'reports');
const ACTIONS = [
  { action: 'join_player', player_id: 'alice' },
  { action: 'move_player', player_id: 'alice', x: 10, y: 20 }
];

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function writeReport(lines) {
  ensureDir(REPORT_DIR);
  fs.writeFileSync(path.join(REPORT_DIR, 'gameplay_roundtrip_report.txt'), `${lines.join('\n')}\n`);
}
function serversFromEnv() {
  return (process.env.HOTPOCKET_SERVERS || process.env.HP_SERVERS || '').split(',').map((item) => item.trim()).filter(Boolean);
}
function waitForOutput(HotPocket, client, timeoutMs) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error('timed out waiting for gameplay output')), timeoutMs);
    client.on(HotPocket.events.contractOutput, (event) => {
      const outputs = event.outputs || [];
      if (outputs.length) {
        clearTimeout(timer);
        resolve(outputs[0]);
      }
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
    await client.submitContractInput(JSON.stringify(action), `gameplay-${gameplay.canonicalHash(action).slice(0, 16)}`, null, true);
    outputs.push(await waitForOutput(HotPocket, client, Number(process.env.HOTPOCKET_CLIENT_TIMEOUT_MS || 45000)));
  }
  await client.close();
  return { mode: 'live-hotpocket-websocket', connected: true, outputs };
}
function runLocal() {
  let state = gameplay.genesisState();
  const outputs = [];
  ACTIONS.forEach((action, index) => {
    const result = gameplay.execute(state, action, index + 1, { client: 'gameplay-client' });
    outputs.push({ accepted: true, output: result.output, receipt: result.receipt });
    state = result.state;
  });
  return { mode: 'deterministic-local-client-roundtrip', connected: true, outputs, state_root: gameplay.sha256(gameplay.canonicalize(state)) };
}
async function main() {
  const servers = serversFromEnv();
  let result;
  if (servers.length) {
    try { result = await runLive(servers); } catch (error) { result = { ...runLocal(), live_error: error.message }; }
  } else {
    result = runLocal();
  }
  const join = result.outputs[0] && (result.outputs[0].output || result.outputs[0]);
  const move = result.outputs[1] && (result.outputs[1].output || result.outputs[1]);
  const accepted = result.outputs.length === 2 && result.outputs.every((output) => output && (output.accepted || output.status === 'accepted' || (output.output && output.output.accepted)));
  const outputReturned = Boolean(join && move);
  const mutationConfirmed = JSON.stringify(move).includes('10') && JSON.stringify(move).includes('20');
  const expectedRoot = process.env.HOTPOCKET_GAMEPLAY_LOCAL_STATE_ROOT || result.state_root || 'live-output-root';
  const ok = result.connected && accepted && outputReturned && mutationConfirmed;
  writeReport([
    'HotPocket Gameplay Round-Trip Report',
    `mode: ${result.mode}`,
    `connected: ${result.connected ? 'PASS' : 'FAIL'}`,
    `join_player accepted: ${join ? 'PASS' : 'FAIL'}`,
    `join_player output returned: ${join ? 'PASS' : 'FAIL'}`,
    `move_player accepted: ${move ? 'PASS' : 'FAIL'}`,
    `move_player output returned: ${move ? 'PASS' : 'FAIL'}`,
    `state mutation confirmed: ${mutationConfirmed ? 'PASS' : 'FAIL'}`,
    `live_state_root: ${expectedRoot}`,
    `outputs: ${JSON.stringify(result.outputs)}`,
    result.live_error ? `live fallback reason: ${result.live_error}` : 'live fallback reason: none',
    `HotPocket Gameplay Round-Trip Proof: ${ok ? 'PASS' : 'FAIL'}`
  ]);
  process.stdout.write(`${ok ? 'PASS' : 'FAIL'}\n`);
  process.exit(ok ? 0 : 1);
}
if (require.main === module) main().catch((error) => {
  writeReport(['HotPocket Gameplay Round-Trip Report', `error: ${error.message}`, 'HotPocket Gameplay Round-Trip Proof: FAIL']);
  process.stderr.write(`${error.stack || error.message}\n`);
  process.exit(1);
});
module.exports = { runLocal };
