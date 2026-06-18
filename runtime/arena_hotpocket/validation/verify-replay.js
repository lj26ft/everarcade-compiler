const assert = require('node:assert/strict');
const { mkdtempSync, rmSync } = require('node:fs');
const { tmpdir } = require('node:os');
const { join } = require('node:path');
const { ArenaVanguard, replayJournal } = require('../src/arena_vanguard');

const inputs = [
  { action: 'join', player: 'player-1' },
  { action: 'join', player: 'player-2' },
  { action: 'move', player: 'player-1', direction: 'east' },
  { action: 'move', player: 'player-2', direction: 'west' },
  { action: 'attack', player: 'player-1', target: 'player-2' }
];
const temp = mkdtempSync(join(tmpdir(), 'arena-hotpocket-replay-'));
(async () => {
  try {
    const app = new ArenaVanguard({ statePath: join(temp, 'arena-wrapper-state.json'), journalPath: join(temp, 'arena-hotpocket-journal.json') });
    for (const [index, input] of inputs.entries()) await app.handleInput(`validator-${index}`, input, { lclSeqNo: index + 1, npl: 1 });
    const replayed = replayJournal(app.journal);
    const live = app.verify().live;
    for (const key of ['state_root', 'receipt_root', 'world_hash', 'continuity_root']) assert.equal(replayed.commitments[key], live[key]);
    console.log(JSON.stringify({ ok: true, ...live }, null, 2));
  } finally {
    rmSync(temp, { recursive: true, force: true });
  }
})().catch((error) => {
  console.error(error);
  process.exit(1);
});
