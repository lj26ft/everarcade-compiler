#!/usr/bin/env node
const [,, action, ...args] = process.argv;
const url = (process.env.ARENA_HOTPOCKET_URL || 'http://127.0.0.1:8787').replace(/\/$/, '');
function usage() { console.error('usage: arena-submit <join|move|attack|disconnect> --player player-1 [--direction north] [--target player-2]'); process.exit(2); }
function flag(name, fallback) { const index = args.indexOf(`--${name}`); return index >= 0 ? args[index + 1] : fallback; }
if (!action) usage();
const player = flag('player', 'player-1');
let input;
if (action === 'join') input = { action, player };
else if (action === 'move') input = { action, player, direction: flag('direction', 'north') };
else if (action === 'attack') input = { action, player, target: flag('target', 'player-2') };
else if (action === 'disconnect') input = { action, player };
else usage();
const response = await fetch(`${url}/input`, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(input) });
const json = await response.json();
console.log(JSON.stringify(json, null, 2));
process.exit(response.ok && json.status !== 'rejected' ? 0 : 1);
