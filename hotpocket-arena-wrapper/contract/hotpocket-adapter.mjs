#!/usr/bin/env node
import { createRequire } from 'node:module';
import { ArenaHotPocketRuntime, inputId, validateEnvelope } from '../src/runtime.mjs';

export const ADAPTER_SCHEMA = 'everarcade.hotpocket.arena-vanguard-adapter.v0.1';
export const runtime = new ArenaHotPocketRuntime({
  statePath: 'state/arena-wrapper-state.json',
  journalPath: 'state/arena-hotpocket-journal.json'
}).load();

function decodePayload(raw) {
  const text = Buffer.isBuffer(raw) ? raw.toString('utf8') : String(raw);
  return JSON.parse(text);
}

export function toArenaEnvelope(input, user, round) {
  const validated = validateEnvelope(input);
  return { ...validated, hotpocket: { round, user: user.publicKey || user.id || 'hotpocket-user' } };
}

export async function readUserInputs(ctx) {
  const users = ctx.users.list();
  const round = Number(ctx.lclSeqNo);
  const npl = ctx.npl;
  const accepted = [];
  const rejected = [];
  for (const user of users) {
    for (const inputRef of user.inputs || []) {
      try {
        const raw = await ctx.users.read(inputRef);
        const payload = decodePayload(raw);
        accepted.push({ user, inputRef, envelope: toArenaEnvelope(payload, user, round), npl });
      } catch (error) {
        rejected.push({ user, inputRef, error: error.message, round });
      }
    }
  }
  return { round, npl, accepted, rejected };
}

export async function handleContext(ctx) {
  const { round, npl, accepted, rejected } = await readUserInputs(ctx);
  const outputs = [];
  for (const item of accepted) {
    try {
      const result = runtime.processAtRound(item.envelope, round);
      const payload = { schema: ADAPTER_SCHEMA, status: 'accepted', input_id: inputId(result.journal.action), round, npl, receipt: result.receipt, journal: result.journal, output: result.output, commitments: result.commitments };
      await item.user.send?.(payload);
      outputs.push(payload);
    } catch (error) {
      const payload = { schema: ADAPTER_SCHEMA, status: 'rejected', round, npl, error: error.message };
      await item.user.send?.(payload);
      outputs.push(payload);
    }
  }
  for (const item of rejected) {
    const payload = { schema: ADAPTER_SCHEMA, status: 'rejected', round, npl, error: item.error };
    await item.user.send?.(payload);
    outputs.push(payload);
  }
  if (outputs.length === 0) runtime.persist();
  return outputs;
}

export async function startHotPocketContract() {
  const require = createRequire(import.meta.url);
  const HotPocket = require('hotpocket-nodejs-contract');
  const hpc = new HotPocket.Contract();
  return hpc.init(async (ctx) => handleContext(ctx), HotPocket.clientProtocols?.json, false);
}

if (import.meta.url === `file://${process.argv[1]}`) {
  startHotPocketContract().then((started) => { if (!started) process.exit(1); }).catch((error) => { console.error(error.stack || error.message); process.exit(1); });
}
