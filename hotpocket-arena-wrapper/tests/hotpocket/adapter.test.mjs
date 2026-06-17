import test from 'node:test';
import assert from 'node:assert/strict';
import { ArenaHotPocketRuntime, replayJournal } from '../../src/runtime.mjs';
import { readUserInputs, toArenaEnvelope } from '../../contract/hotpocket-adapter.mjs';

function ctx(round, payloads) {
  const sent = [];
  const refs = payloads.map((payload, i) => ({ id: i, payload: JSON.stringify(payload) }));
  const user = { publicKey: 'user-1', inputs: refs, send: async (value) => sent.push(value) };
  return { lclSeqNo: round, npl: 3, sent, users: { list: () => [user], read: async (ref) => Buffer.from(ref.payload) } };
}

test('ctx.users inputs are parsed into canonical Arena envelopes', async () => {
  const hotpocketCtx = ctx(42, [{ action: 'move', player: 'player-1', direction: 'east' }]);
  const parsed = await readUserInputs(hotpocketCtx);
  assert.equal(parsed.round, 42);
  assert.equal(parsed.npl, 3);
  assert.deepEqual(parsed.accepted[0].envelope, { action: 'move', player: 'player-1', direction: 'east', hotpocket: { round: 42, user: 'user-1' } });
});

test('invalid input is rejected before runtime mutation', async () => {
  const hotpocketCtx = ctx(7, [{ action: 'move', player: 'player-1', direction: 'up' }]);
  const parsed = await readUserInputs(hotpocketCtx);
  assert.equal(parsed.accepted.length, 0);
  assert.equal(parsed.rejected.length, 1);
  assert.match(parsed.rejected[0].error, /direction/);
});

test('round processing uses ctx.lclSeqNo as the arena tick source', () => {
  const runtime = new ArenaHotPocketRuntime();
  runtime.processAtRound(toArenaEnvelope({ action: 'join', player: 'player-1' }, { publicKey: 'u' }, 100), 100);
  const move = runtime.processAtRound(toArenaEnvelope({ action: 'move', player: 'player-1', direction: 'east' }, { publicKey: 'u' }, 101), 101);
  assert.equal(runtime.state.tick, 101);
  assert.equal(move.output.tick, 101);
});

test('commitment generation and replay equivalence remain stable for HotPocket journal', () => {
  const runtime = new ArenaHotPocketRuntime();
  runtime.processAtRound({ action: 'join', player: 'player-1' }, 10);
  runtime.processAtRound({ action: 'join', player: 'player-2' }, 10);
  runtime.processAtRound({ action: 'attack', player: 'player-1', target: 'player-2' }, 11);
  const replayed = replayJournal(runtime.journal);
  for (const key of ['state_root', 'receipt_root', 'world_hash', 'continuity_root']) assert.equal(replayed.commitments[key], runtime.verify().live[key]);
});
