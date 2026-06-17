#!/usr/bin/env node
import { writeFileSync, mkdirSync, rmSync } from 'node:fs';
import { join } from 'node:path';
import { ArenaHotPocketRuntime, defaultPaths } from '../src/runtime.mjs';
const root = process.env.EVERARCADE_REPO_ROOT || process.cwd();
const reportDir = join(root, 'hotpocket-arena-wrapper/reports');
mkdirSync(reportDir, { recursive: true });
rmSync(join(root, 'evernode/hotpocket/arena-wrapper-state.json'), { force: true });
rmSync(join(root, 'evernode/journals/arena-hotpocket-journal.json'), { force: true });
const runtime = new ArenaHotPocketRuntime(defaultPaths(root)).load();
const sequence = [
  { action: 'join', player: 'player-1' },
  { action: 'join', player: 'player-2' },
  { action: 'move', player: 'player-1', direction: 'north' },
  { action: 'attack', player: 'player-1', target: 'player-2' }
];
const outputs = sequence.map((input) => runtime.process(input).output);
const verification = runtime.verify();
const checks = {
  join: outputs[0].players['player-1'].connected === true,
  move: outputs[2].players['player-1'].y === -1,
  attack: outputs[3].players['player-2'].health === 75,
  journal: runtime.journal.length === sequence.length,
  state_root: verification.live.state_root === verification.replayed.state_root,
  receipt_root: verification.live.receipt_root === verification.replayed.receipt_root,
  world_hash: verification.live.world_hash === verification.replayed.world_hash,
  continuity_root: verification.live.continuity_root === verification.replayed.continuity_root
};
const ok = Object.values(checks).every(Boolean) && verification.ok;
writeFileSync(join(reportDir, 'live-path-verification.json'), `${JSON.stringify({ schema: 'everarcade.hotpocket.arena-wrapper.validation.v0.1', sequence, outputs, checks, verification, status: ok ? 'PASS' : 'FAIL' }, null, 2)}\n`);
console.log(`HotPocket Arena Wrapper Live Path: ${ok ? 'PASS' : 'FAIL'}`);
process.exit(ok ? 0 : 1);
