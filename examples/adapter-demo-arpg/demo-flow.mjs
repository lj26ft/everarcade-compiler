import { createArpgWorld } from '../../adapters/browser-arpg/adapter.js';

const adapter = createArpgWorld();
let frame = adapter.connectWorld();
const inputs = [
  { type: 'move', dx: 240, dy: -80 },
  { type: 'attack' }, { type: 'attack' }, { type: 'pickup' },
  { type: 'move', dx: 160, dy: 80 },
  { type: 'attack' }, { type: 'attack' }, { type: 'pickup' },
  { type: 'move', dx: 130, dy: 120 },
  { type: 'attack' }, { type: 'attack' }, { type: 'pickup' },
  { type: 'open_chest' }, { type: 'restore' }, { type: 'migrate' }, { type: 'return_later' }
];
for (const input of inputs) frame = adapter.submitInput(input);
const verification = adapter.verifyWorld();
console.log(JSON.stringify({ status: verification.status, receipts: verification.receipts, inventory: frame.inventory, memory: frame.memory, roots: verification.roots }, null, 2));
if (verification.status !== 'PASS') process.exit(1);
if (!frame.memory.includes('Dungeon room cleared.')) process.exit(1);
if (frame.inventory.length < 3) process.exit(1);
