'use strict';

const fs = require('fs');
const path = require('path');
const { canonicalHash } = require('../payload_builder');

const ROOT = path.resolve(__dirname, '..');
const LEDGER_DIR = path.join(ROOT, 'ledgers');

function ensureDir(dir) { fs.mkdirSync(dir, { recursive: true }); }
function writeJson(file, value) { ensureDir(path.dirname(file)); fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`); }

function constructTransaction(network, payloadBundle) {
  const namespace = network === 'xahau' ? 'everarcade.xahau.anchor.v0.1' : 'everarcade.xrpl.anchor.v0.1';
  const transaction = {
    TransactionType: network === 'xahau' ? 'Invoke' : 'Payment',
    Account: `rEverArcade${network.toUpperCase()}AnchorPublisher`,
    Destination: `rEverArcade${network.toUpperCase()}AnchorRegistry`,
    Fee: '12',
    NetworkID: network,
    Memos: [
      {
        Memo: {
          MemoType: Buffer.from(namespace).toString('hex'),
          MemoData: Buffer.from(payloadBundle.canonical_payload, 'utf8').toString('hex')
        }
      }
    ],
    LastLedgerSequence: 1,
    SigningPubKey: 'deterministic-publication-proof',
    TxnSignature: canonicalHash({ network, payload_hash: payloadBundle.payload_hash, signer: 'everarcade-anchor-publisher' })
  };
  return { ...transaction, hash: canonicalHash(transaction) };
}

function submit(network, payloadBundle) {
  if (!['xrpl', 'xahau'].includes(network)) throw new Error(`unsupported anchor network: ${network}`);
  const transaction = constructTransaction(network, payloadBundle);
  const ledgerEntry = {
    schema: `everarcade.${network}.anchor-publication-ledger-entry.v0.1`,
    network,
    ledger_index: 1,
    engine_result: 'tesSUCCESS',
    accepted: true,
    validated: true,
    payload: payloadBundle.payload,
    payload_hash: payloadBundle.payload_hash,
    transaction,
    transaction_hash: transaction.hash,
    close_time: 0
  };
  writeJson(path.join(LEDGER_DIR, `${network}-ledger-entry.json`), ledgerEntry);
  return ledgerEntry;
}

module.exports = { constructTransaction, submit };
