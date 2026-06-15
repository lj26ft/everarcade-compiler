#!/usr/bin/env node
'use strict';

const { createTransportSubmission, processAcceptedSubmission, RUNTIME_PROTOCOL } = require('../src');

let runtimeState = { mutations: [], sequence: 0 };

async function readInput(ctx, inputRef) {
  const raw = await ctx.users.read(inputRef);
  return Buffer.isBuffer(raw) ? raw : Buffer.from(String(raw));
}

async function handleContext(ctx) {
  const users = ctx && ctx.users && typeof ctx.users.list === 'function' ? ctx.users.list() : [];
  const receipts = [];
  for (const user of users) {
    for (const inputRef of user.inputs || []) {
      const rawPayload = await readInput(ctx, inputRef);
      const submission = createTransportSubmission({
        lease_id: process.env.EVERARCADE_LEASE_ID || ctx.leaseId || '',
        contract_id: process.env.EVERARCADE_CONTRACT_ID || ctx.contractId || '',
        user_public_key: user.publicKey || '',
        input_hash: inputRef.hash || inputRef.inputHash || undefined,
        nonce: inputRef.nonce || 0,
        ledger_seq_no: ctx.lclSeqNo ?? null,
        raw_payload: rawPayload
      });
      const result = processAcceptedSubmission(submission, runtimeState);
      runtimeState = result.state;
      const output = { protocol: RUNTIME_PROTOCOL, receipt: result.receipt };
      await user.send(output);
      console.log(JSON.stringify({ event: 'everarcade_transport_receipt', input_hash: submission.input_hash, ledger_seq_no: submission.ledger_seq_no, user_public_key: submission.user_public_key, receipt_root: result.receipt.receipt_root }));
      receipts.push(output);
    }
  }
  return receipts;
}

async function main() {
  const HotPocket = require('hotpocket-nodejs-contract');
  const contract = new HotPocket.Contract();
  const started = await contract.init(handleContext, HotPocket.clientProtocols.json, false);
  if (!started) process.exit(1);
}

module.exports = { handleContext };
if (require.main === module) main().catch((error) => { console.error(error.stack || error.message); process.exit(1); });
