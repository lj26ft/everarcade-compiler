#!/usr/bin/env node
'use strict';

const fs = require('fs');
const path = require('path');
const {
  executeInput,
  ensureDir,
  loadState,
  saveState,
  writeJson
} = require('../src/artifacts');

const PROOF_ROOT = path.resolve(__dirname, '..');
const REPORT_DIR = process.env.EVERARCADE_HOTPOCKET_REPORT_DIR || path.join(PROOF_ROOT, 'reports');
const STATE_DIR = process.env.EVERARCADE_HOTPOCKET_STATE_DIR || path.join(PROOF_ROOT, '.state');

function inspectContext(ctx) {
  const users = ctx && ctx.users;
  const unl = ctx && ctx.unl;
  const report = {
    schema: 'everarcade.hotpocket.live-context-inspection.v0.1',
    originated_from: 'contract invocation context',
    contract_id: ctx && ctx.contractId,
    public_key_present: Boolean(ctx && ctx.publicKey),
    readonly: Boolean(ctx && ctx.readonly),
    timestamp_type: typeof (ctx && ctx.timestamp),
    lcl_seq_no_type: typeof (ctx && ctx.lclSeqNo),
    lcl_hash_type: typeof (ctx && ctx.lclHash),
    context_keys: ctx ? Object.keys(ctx).sort() : [],
    users_surface: users ? Object.getOwnPropertyNames(Object.getPrototypeOf(users)).filter((k) => k !== 'constructor').sort() : [],
    user_count: users && typeof users.count === 'function' ? users.count() : null,
    unl_surface: unl ? Object.getOwnPropertyNames(Object.getPrototypeOf(unl)).filter((k) => k !== 'constructor').sort() : [],
    unl_count: unl && typeof unl.count === 'function' ? unl.count() : null
  };
  ensureDir(REPORT_DIR);
  writeJson(path.join(REPORT_DIR, 'hotpocket_live_context_report.json'), report);
  return report;
}

async function readUserInput(ctx, user, inputRef) {
  const raw = await ctx.users.read(inputRef);
  return raw.toString('utf8');
}

async function handleContext(ctx) {
  inspectContext(ctx);
  const state = loadState(STATE_DIR);
  const users = ctx.users.list();
  const outputs = [];

  for (const user of users) {
    for (const inputRef of user.inputs) {
      const raw = await readUserInput(ctx, user, inputRef);
      const result = executeInput(state, raw, {
        ledger_seq: ctx.lclSeqNo || 0,
        timestamp: ctx.timestamp || undefined,
        user: user.publicKey
      });
      const payload = {
        status: 'ok',
        execution_id: result.receipt.execution_id,
        output: result.output,
        receipt: result.receipt,
        journal: result.journal,
        checkpoint: result.checkpoint
      };
      await user.send(payload);
      outputs.push(payload);
    }
  }

  saveState(STATE_DIR, state);
  writeJson(path.join(REPORT_DIR, 'hotpocket_last_execution_artifacts.json'), {
    schema: 'everarcade.hotpocket.live-execution-artifacts.v0.1',
    output_count: outputs.length,
    outputs,
    roots: {
      state_root: state.state_root,
      receipt_root: state.receipt_root,
      journal_root: state.journal_root
    }
  });
  return outputs;
}

async function main() {
  const HotPocket = require('hotpocket-nodejs-contract');
  const contract = new HotPocket.Contract();
  const started = await contract.init(handleContext, HotPocket.clientProtocols.json, false);
  if (!started) process.exit(1);
}

module.exports = { handleContext, inspectContext };

if (require.main === module) {
  main().catch((error) => {
    process.stderr.write(`${error.stack || error.message}\n`);
    process.exit(1);
  });
}
