#!/usr/bin/env node
'use strict';

const path = require('path');
const HotPocket = require('hotpocket-js-client');
const {
  DEFAULT_TIME,
  canonicalHash,
  ensureDir,
  writeJson
} = require('../src/artifacts');

const PROOF_ROOT = path.resolve(__dirname, '..');
const REPORT_DIR = process.env.EVERARCADE_HOTPOCKET_REPORT_DIR || path.join(PROOF_ROOT, 'reports');

function serversFromEnv() {
  const raw = process.env.HOTPOCKET_SERVERS || process.env.HP_SERVERS || '';
  return raw.split(',').map((server) => server.trim()).filter(Boolean);
}

function waitForOutput(client, submissionHash, timeoutMs) {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(`timed out waiting for HotPocket output for ${submissionHash}`)), timeoutMs);
    client.on(HotPocket.events.contractOutput, (event) => {
      const outputs = event.outputs || [];
      const match = outputs.find((output) => {
        const candidate = output && (output.submission_hash || output.submissionHash || (output.receipt && output.receipt.input_hash));
        return !candidate || candidate === submissionHash;
      });
      if (match) {
        clearTimeout(timer);
        resolve({ event, output: match });
      }
    });
  });
}

async function runRoundTrip(options = {}) {
  const servers = options.servers || serversFromEnv();
  if (!servers.length) throw new Error('HOTPOCKET_SERVERS or HP_SERVERS must list at least one ws:// endpoint');
  const payload = options.payload || { action: process.env.HOTPOCKET_ACTION || 'ping' };
  const timeoutMs = Number(process.env.HOTPOCKET_CLIENT_TIMEOUT_MS || options.timeoutMs || 45000);
  const clientKeys = process.env.HOTPOCKET_CLIENT_PRIVATE_KEY
    ? HotPocket.generateKeys(process.env.HOTPOCKET_CLIENT_PRIVATE_KEY)
    : HotPocket.generateKeys();
  const client = await HotPocket.createClient(servers, clientKeys, {
    contractId: process.env.HOTPOCKET_CONTRACT_ID || null,
    contractVersion: process.env.HOTPOCKET_CONTRACT_VERSION || null,
    trustedServerKeys: process.env.HOTPOCKET_TRUSTED_SERVER_KEYS ? process.env.HOTPOCKET_TRUSTED_SERVER_KEYS.split(',') : null,
    requiredConnectionCount: Number(process.env.HOTPOCKET_REQUIRED_CONNECTIONS || 1),
    connectionTimeoutMs: Number(process.env.HOTPOCKET_CONNECTION_TIMEOUT_MS || 10000)
  });

  const started = Date.now();
  const connected = await client.connect();
  if (!connected) throw new Error('HotPocket client did not satisfy required connection count');
  client.subscribe(HotPocket.notificationChannels.ledgerEvent);

  const nonce = process.env.HOTPOCKET_INPUT_NONCE || `everarcade-proof-${canonicalHash(JSON.stringify(payload)).slice(0, 16)}`;
  const submitResults = await client.submitContractInput(JSON.stringify(payload), nonce, null, true);
  const submission = submitResults && submitResults[0] ? submitResults[0] : {};
  const submissionHash = submission.hash || submission.input_hash || canonicalHash({ payload, nonce });
  const completion = await waitForOutput(client, submissionHash, timeoutMs);
  await client.close();

  const completed = Date.now();
  const report = {
    schema: 'everarcade.hotpocket.client-roundtrip.v0.1',
    generated_at: DEFAULT_TIME,
    servers,
    connected: true,
    submission_hash: submissionHash,
    submission_status: submission.status || 'submitted',
    output_payload: completion.output,
    completion_time_ms: completed - started,
    verified: completion.output && completion.output.output && completion.output.output.status === 'ok'
  };
  ensureDir(REPORT_DIR);
  writeJson(path.join(REPORT_DIR, 'hotpocket_client_roundtrip_result.json'), report);
  return report;
}

if (require.main === module) {
  runRoundTrip().then((report) => {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    process.exit(report.verified ? 0 : 1);
  }).catch((error) => {
    const report = { schema: 'everarcade.hotpocket.client-roundtrip.v0.1', generated_at: DEFAULT_TIME, connected: false, verified: false, error: error.message };
    ensureDir(REPORT_DIR);
    writeJson(path.join(REPORT_DIR, 'hotpocket_client_roundtrip_result.json'), report);
    process.stderr.write(`${error.stack || error.message}\n`);
    process.exit(1);
  });
}

module.exports = { runRoundTrip };
