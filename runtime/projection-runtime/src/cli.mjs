#!/usr/bin/env node
import { loadJournal, projectArenaState, replayJournal } from './runtime.mjs';

const args = Object.fromEntries(process.argv.slice(2).map((arg, index, list) => arg.startsWith('--') ? [arg.slice(2), list[index + 1]?.startsWith('--') ? true : list[index + 1]] : []));
const journal = loadJournal(args.journal ?? '../../examples/projection-demo-world/journal.json');
const mode = args.mode ?? 'live';
const frames = mode === 'replay'
  ? replayJournal(journal, { operator: args.operator ?? 'Operator A' })
  : journal.ticks.map((tick) => projectArenaState(tick.arenaState, { operator: args.operator ?? 'Operator A', mode: 'live' }));
for (const frame of frames) {
  console.log(JSON.stringify({ mode: frame.mode, operator: frame.operator, epoch: frame.epoch, tick: frame.tick, root: frame.root, players: frame.players.length, entities: frame.entities.length, receipts: frame.receipt_count }));
}
