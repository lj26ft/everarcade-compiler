#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';
import { Client, Wallet, xrpToDrops, convertStringToHex, convertHexToString } from 'xrpl';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(__dirname, '..');
const reportDir = process.env.EVERARCADE_PROOF_REPORT_DIR || path.join(packageRoot, 'reports');
const artifactDir = process.env.EVERARCADE_PROOF_ARTIFACT_DIR || path.join(packageRoot, 'artifacts');
const endpoint = process.env.XRPL_TESTNET_ENDPOINT || 'wss://s.altnet.rippletest.net:51233';

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function readJson(file) { return JSON.parse(fs.readFileSync(file, 'utf8')); }
function writeJson(file, value) { ensureDir(path.dirname(file)); fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`); }
function canonical(value) {
  if (value === null || typeof value !== 'object') return JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
  return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(',')}}`;
}
function hashJson(value) { return crypto.createHash('sha256').update(canonical(value)).digest('hex'); }
function report(status, extra) {
  writeJson(path.join(reportDir, 'lease_xrpl_anchor_report.json'), {
    schema: 'everarcade.evernode.xrpl-anchor-report.v0.1',
    generated_at: new Date().toISOString(),
    network: 'xrpl-testnet',
    endpoint,
    ...extra,
    status
  });
}

async function main() {
  const seed = process.env.XRPL_TESTNET_SEED;
  const destination = process.env.XRPL_TESTNET_DESTINATION;
  if (!seed || !destination) {
    report('FAIL', {
      submission: 'FAIL',
      ledger_acceptance: 'FAIL',
      retrieval: 'FAIL',
      hash_equivalence: 'FAIL',
      failure: 'XRPL_TESTNET_SEED and XRPL_TESTNET_DESTINATION are required for real XRPL Testnet publication.'
    });
    process.exit(1);
  }
  const anchor = readJson(path.join(artifactDir, 'continuity-anchor.json'));
  const payload = {
    schema: 'everarcade.evernode.xrpl-anchor-payload.v0.1',
    lease_id: anchor.lease_id,
    host_id: anchor.host_id,
    continuity_root: anchor.continuity_root,
    anchor_hash: anchor.anchor_hash,
    state_root: anchor.state_root,
    replay_root: anchor.replay_root
  };
  const payloadHash = hashJson(payload);
  const memoData = JSON.stringify({ protocol: 'everarcade-evernode-proof-v0.1', payload_hash: payloadHash, anchor_hash: anchor.anchor_hash, continuity_root: anchor.continuity_root });

  const wallet = Wallet.fromSeed(seed);
  const client = new Client(endpoint);
  await client.connect();
  try {
    const prepared = await client.autofill({
      TransactionType: 'Payment',
      Account: wallet.address,
      Destination: destination,
      Amount: xrpToDrops(process.env.XRPL_TESTNET_AMOUNT_XRP || '0.000001'),
      Memos: [{ Memo: { MemoType: convertStringToHex('everarcade.evernode.proof.v0.1'), MemoData: convertStringToHex(memoData) } }]
    });
    const signed = wallet.sign(prepared);
    const result = await client.submitAndWait(signed.tx_blob);
    const txHash = signed.hash;
    const accepted = result.result.meta?.TransactionResult === 'tesSUCCESS';
    const tx = await client.request({ command: 'tx', transaction: txHash, binary: false });
    const memoHex = tx.result.Memos?.[0]?.Memo?.MemoData;
    const retrievedMemo = memoHex ? JSON.parse(convertHexToString(memoHex)) : null;
    const hashMatch = retrievedMemo?.payload_hash === payloadHash && retrievedMemo?.anchor_hash === anchor.anchor_hash && retrievedMemo?.continuity_root === anchor.continuity_root;
    report(accepted && hashMatch ? 'PASS' : 'FAIL', {
      submission: 'PASS',
      ledger_acceptance: accepted ? 'PASS' : 'FAIL',
      retrieval: retrievedMemo ? 'PASS' : 'FAIL',
      hash_equivalence: hashMatch ? 'PASS' : 'FAIL',
      engine_result: result.result.meta?.TransactionResult,
      transaction_hash: txHash,
      ledger_index: result.result.ledger_index,
      payload_hash: payloadHash,
      anchor_hash: anchor.anchor_hash,
      continuity_root: anchor.continuity_root,
      retrieved_memo: retrievedMemo
    });
    if (!accepted || !hashMatch) process.exit(1);
  } finally {
    await client.disconnect();
  }
}

main().catch((error) => {
  report('FAIL', {
    submission: 'FAIL',
    ledger_acceptance: 'FAIL',
    retrieval: 'FAIL',
    hash_equivalence: 'FAIL',
    failure: error.message
  });
  console.error(error);
  process.exit(1);
});
