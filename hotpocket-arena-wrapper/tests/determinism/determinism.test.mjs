import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import {
  ArenaHotPocketRuntime,
  canonicalHash,
  canonicalize,
  commitFor,
  defaultPaths,
  genesisState,
  inputId,
  replayJournal
} from '../../src/runtime.mjs';

const sequence = Object.freeze([
  { action: 'join', player: 'player-1' },
  { action: 'join', player: 'player-2' },
  { action: 'move', player: 'player-1', direction: 'north' },
  { action: 'attack', player: 'player-1', target: 'player-2' },
  { action: 'disconnect', player: 'player-2' }
]);
const commitmentKeys = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'];

function runLive(root) {
  const runtime = new ArenaHotPocketRuntime(defaultPaths(root)).load();
  for (const input of sequence) runtime.process(input);
  return runtime;
}

function commitments(runtime) {
  const latest = runtime.state.commitments.at(-1) || commitFor(runtime.state, runtime.receipts);
  return Object.fromEntries(commitmentKeys.map((key) => [key, latest[key]]));
}

test('deterministic input IDs are stable for identical canonical input', () => {
  const first = { player: 'player-1', action: 'join' };
  const second = { action: 'join', player: 'player-1' };
  assert.equal(inputId(first), inputId(second));
  assert.equal(inputId(first), `arena-${canonicalHash(first)}`);
});

test('serialization stability uses sorted canonical bytes regardless of insertion order', () => {
  const a = { z: 3, nested: { b: 2, a: 1 }, list: [{ y: true, x: false }] };
  const b = { list: [{ x: false, y: true }], nested: { a: 1, b: 2 }, z: 3 };
  assert.equal(canonicalize(a), canonicalize(b));
  assert.equal(canonicalHash(a), canonicalHash(b));
});

test('root stability: replaying the same journal twice yields identical commitments', () => {
  const root = mkdtempSync(join(tmpdir(), 'arena-determinism-root-'));
  try {
    const runtime = runLive(root);
    const replayA = replayJournal(runtime.journal);
    const replayB = replayJournal(runtime.journal);
    assert.deepEqual(replayA.commitments, replayB.commitments);
    for (const key of commitmentKeys) assert.equal(replayA.commitments[key], commitments(runtime)[key]);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('replay equivalence: live execution and replay execution match all commitments', () => {
  const runtime = new ArenaHotPocketRuntime();
  for (const input of sequence) runtime.process(input);
  const verification = runtime.verify();
  assert.equal(verification.ok, true);
  for (const key of commitmentKeys) assert.equal(verification.live[key], verification.replayed[key]);
});

test('persisted-state rebuild preserves commitments and roots', () => {
  const root = mkdtempSync(join(tmpdir(), 'arena-determinism-persist-'));
  try {
    const live = runLive(root);
    const loaded = new ArenaHotPocketRuntime(defaultPaths(root)).load();
    assert.deepEqual(commitments(loaded), commitments(live));
    assert.equal(loaded.verify().ok, true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('different-machine simulation: independent roots with same journal match commitments', () => {
  const machineA = new ArenaHotPocketRuntime();
  const machineB = new ArenaHotPocketRuntime();
  for (const input of sequence) {
    machineA.process(input);
    machineB.process({ ...input });
  }
  assert.deepEqual(commitments(machineA), commitments(machineB));
  assert.notEqual(machineA.state, machineB.state);
  assert.deepEqual(machineA.journal.map((entry) => entry.journal_hash), machineB.journal.map((entry) => entry.journal_hash));
});

test('genesis commitments are deterministic', () => {
  assert.deepEqual(commitFor(genesisState(), []), commitFor(genesisState(), []));
});
