const test = require('node:test');
const assert = require('node:assert/strict');
const { existsSync, readFileSync, mkdtempSync, rmSync } = require('node:fs');
const { join } = require('node:path');
const { tmpdir } = require('node:os');
const { ArenaVanguard, replayJournal } = require('../src/arena_vanguard');

const root = join(__dirname, '..');
const fiveInputs = Object.freeze([
  { action: 'join', player: 'player-1' },
  { action: 'join', player: 'player-2' },
  { action: 'move', player: 'player-1', direction: 'east' },
  { action: 'move', player: 'player-2', direction: 'west' },
  { action: 'attack', player: 'player-1', target: 'player-2' }
]);
const rootKeys = ['state_root', 'receipt_root', 'world_hash', 'continuity_root'];

function read(relativePath) {
  return readFileSync(join(root, relativePath), 'utf8');
}

test('hpdevkit package entry points exist', () => {
  assert.equal(existsSync(join(root, 'package.json')), true);
  assert.equal(existsSync(join(root, 'src/contract.js')), true);
  assert.equal(existsSync(join(root, 'src/arena_vanguard.js')), true);
  assert.equal(existsSync(join(root, 'dist/hp.cfg.override')), true);
});

test('contract entry uses HotPocket user input path and lclSeqNo tick source', () => {
  const source = read('src/contract.js');
  assert.match(source, /require\(['"]hotpocket-nodejs-contract['"]\)/);
  assert.match(source, /new HotPocket\.Contract\(\)/);
  assert.match(source, /hpc\.init\(contract\)/);
  assert.match(source, /ctx\.users\.list\(\)/);
  assert.match(source, /ctx\.users\.read\(input\)/);
  assert.match(source, /lclSeqNo:\s*ctx\.lclSeqNo/);
});

test('canonical five-input sequence produces stable roots and replay equivalence', async () => {
  const temp = mkdtempSync(join(tmpdir(), 'arena-hotpocket-package-'));
  try {
    const app = new ArenaVanguard({ statePath: join(temp, 'arena-wrapper-state.json'), journalPath: join(temp, 'arena-hotpocket-journal.json') });
    const outputs = [];
    for (const [index, input] of fiveInputs.entries()) outputs.push(await app.handleInput(`user-${index + 1}`, input, { lclSeqNo: index + 1, npl: 1, readonly: false }));
    const latest = outputs.at(-1).commitments;
    for (const key of rootKeys) assert.equal(typeof latest[key], 'string');
    assert.deepEqual(latest, {
      tick: 5,
      state_root: '86ecb78930d0be6e1487f8731586d1588e2266b4fb7b110c51493646ab20666e',
      receipt_root: 'b3b09e6045b05170e26241bd196da0cd33a33d158e27500d3d31a9d158107f16',
      world_hash: '3cbad7b436ef545c3e44e72b92d0bc9e9a2b67b4af84f468027066535d0e4102',
      continuity_root: 'df128e8d12b5c65fdca9bc404d694a3621c4d14a8369bd0675fe2eea77961784'
    });
    const replayed = replayJournal(app.journal);
    for (const key of rootKeys) assert.equal(replayed.commitments[key], latest[key]);
    assert.equal(app.verify().ok, true);
  } finally {
    rmSync(temp, { recursive: true, force: true });
  }
});

test('rejected validation does not mutate persisted journal', async () => {
  const temp = mkdtempSync(join(tmpdir(), 'arena-hotpocket-reject-'));
  try {
    const app = new ArenaVanguard({ statePath: join(temp, 'arena-wrapper-state.json'), journalPath: join(temp, 'arena-hotpocket-journal.json') });
    await assert.rejects(() => app.handleInput('user-1', { action: 'move', player: 'player-1', direction: 'up' }, { lclSeqNo: 1 }), /direction/);
    assert.equal(app.journal.length, 0);
    assert.equal(existsSync(join(temp, 'arena-wrapper-state.json')), false);
  } finally {
    rmSync(temp, { recursive: true, force: true });
  }
});
