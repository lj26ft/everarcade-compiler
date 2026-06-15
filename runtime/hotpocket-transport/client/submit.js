#!/usr/bin/env node
'use strict';

const { awaitAcceptedSubmission, canonicalHash } = require('../src');

async function submitAccepted(client, payload, options = {}) {
  const nonce = options.nonce || `everarcade-${canonicalHash(payload).slice(0, 16)}`;
  const result = await client.submitContractInput(JSON.stringify(payload), nonce, options.maxLedger ?? null, true);
  const status = await awaitAcceptedSubmission(result);
  if (!status.accepted) {
    console.error(JSON.stringify({ event: 'everarcade_submission_rejected', input_hash: status.hash || null, status: status.status, reason: status.reason, server: status.server, ledger_seq_no: status.ledger_seq_no ?? null }));
    return { accepted: false, status };
  }
  console.log(JSON.stringify({ event: 'everarcade_submission_accepted', input_hash: status.hash, status: status.status, server: status.server, ledger_seq_no: status.ledger_seq_no ?? null }));
  return { accepted: true, status };
}

module.exports = { submitAccepted };
