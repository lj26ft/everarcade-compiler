'use strict';

const fs = require('fs');
const path = require('path');
const { canonicalHash } = require('../payload_builder');

const ROOT = path.resolve(__dirname, '..');
const LEDGER_DIR = path.join(ROOT, 'ledgers');

function readJson(file) { return JSON.parse(fs.readFileSync(file, 'utf8')); }

function retrieve(network) {
  const entry = readJson(path.join(LEDGER_DIR, `${network}-ledger-entry.json`));
  const retrievedPayloadHash = canonicalHash(entry.payload);
  return {
    network,
    transaction_hash: entry.transaction_hash,
    accepted: entry.accepted === true && entry.engine_result === 'tesSUCCESS' && entry.validated === true,
    retrieved_payload: entry.payload,
    retrieved_payload_hash: retrievedPayloadHash,
    ledger_payload_hash: entry.payload_hash,
    retrieval_match: retrievedPayloadHash === entry.payload_hash
  };
}

module.exports = { retrieve };
