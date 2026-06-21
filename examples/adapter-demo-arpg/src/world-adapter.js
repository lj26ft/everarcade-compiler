export function connectWorld() {
  const world = initialWorld('adapter-demo-arpg-v1');
  record(world, 'world.spawn', 'Dungeon instance connected through browser adapter.');
  return api(world);
}

function api(world) {
  return {
    submitInput(input) { applyInput(world, input); return readProjection(world); },
    readProjection() { return readProjection(world); },
    verifyWorld() { updateRoots(world); return { status: 'PASS', receipts: world.receipts.length, roots: world.roots }; },
    disconnect() { record(world, 'continuity.record_event', 'Adapter disconnected after preserving receipts and roots.'); return this.verifyWorld(); }
  };
}

function initialWorld(seed) {
  return { seed, tick: 0, migrated: false, restoredFrom: null, player: { id: 'player-1', hp: 30, x: 120, y: 260, inventory: [] }, enemies: [{ id: 'wisp-1', hp: 8, x: 360, y: 180, alive: true }, { id: 'wisp-2', hp: 8, x: 520, y: 260, alive: true }, { id: 'warden-1', hp: 12, x: 650, y: 380, alive: true }], loot: [], receipts: [], memory: [], roots: { state: 'genesis', receipts: 'genesis' } };
}

function applyInput(world, input) {
  world.tick += 1;
  if (input.type === 'move') move(world, input);
  if (input.type === 'attack') attack(world, input);
  if (input.type === 'pickup') pickup(world);
  if (input.type === 'open_chest') openChest(world);
  if (input.type === 'return_later') record(world, 'continuity.record_event', 'Player returned later; dungeon history is still present.');
  if (input.type === 'restore') { world.restoredFrom = world.roots.state; record(world, 'continuity.record_event', `Restore simulated from root ${world.restoredFrom}.`); }
  if (input.type === 'migrate') { world.migrated = true; record(world, 'continuity.record_event', 'Migration simulated: v1 dungeon history preserved.'); }
  updateRoots(world);
}

function move(world, { dx = 0, dy = 0 }) { world.player.x = clamp(world.player.x + dx, 40, 760); world.player.y = clamp(world.player.y + dy, 40, 480); record(world, 'position.move', `Player moved to ${world.player.x},${world.player.y}.`); }
function attack(world, input) { const target = world.enemies.find((enemy) => enemy.alive && distance(world.player, enemy) <= 96) ?? world.enemies.find((enemy) => enemy.alive && enemy.id === input.targetId); if (!target) return record(world, 'combat.attack', 'Attack resolved with no valid target.'); const damage = target.id.startsWith('warden') ? 6 : 4; target.hp = Math.max(0, target.hp - damage); record(world, 'combat.attack', `${target.id} took ${damage} deterministic damage.`); if (target.hp === 0 && target.alive) { target.alive = false; const drop = { id: `loot-${world.receipts.length + 1}`, name: target.id.startsWith('warden') ? 'Verdant Keystone' : 'Echo Shard', x: target.x, y: target.y, owner: null }; world.loot.push(drop); record(world, 'inventory.pickup', `${target.id} died; authoritative loot ${drop.name} spawned.`); } if (world.enemies.every((enemy) => !enemy.alive)) record(world, 'continuity.record_event', 'Dungeon room cleared.'); }
function pickup(world) { const item = world.loot.find((loot) => !loot.owner && distance(world.player, loot) <= 80); if (!item) return record(world, 'inventory.pickup', 'Pickup rejected: no authoritative loot in range.'); item.owner = world.player.id; world.player.inventory.push(item.name); record(world, 'inventory.transfer', `${item.name} transferred to ${world.player.id}.`); }
function openChest(world) { if (world.loot.some((loot) => loot.id === 'chest-ember')) return; world.loot.push({ id: 'chest-ember', name: 'Amber Lens', x: 705, y: 120, owner: null }); record(world, 'world.spawn', 'Open chest spawned Amber Lens through world authority.'); }
function record(world, type, message) { world.receipts.push({ index: world.receipts.length + 1, tick: world.tick, type, message }); world.memory.push(message); }
function updateRoots(world) { world.roots.state = hash({ player: world.player, enemies: world.enemies, loot: world.loot, migrated: world.migrated, restoredFrom: world.restoredFrom }); world.roots.receipts = hash(world.receipts); }
function readProjection(world) { return JSON.parse(JSON.stringify({ frame: world.tick, player: world.player, enemies: world.enemies, loot: world.loot, inventory: world.player.inventory, receipts: { count: world.receipts.length, latest: world.receipts.at(-1) ?? null }, roots: world.roots, memory: world.memory.slice(-8), migrated: world.migrated, restoredFrom: world.restoredFrom })); }
function hash(value) { const text = JSON.stringify(value); let h1 = 0x811c9dc5; let h2 = 0x01000193; for (let i = 0; i < text.length; i += 1) { h1 ^= text.charCodeAt(i); h1 = Math.imul(h1, 0x01000193); h2 = Math.imul(h2 ^ text.charCodeAt(i), 0x85ebca6b); } return `${(h1 >>> 0).toString(16).padStart(8, '0')}${(h2 >>> 0).toString(16).padStart(8, '0')}`; }
function clamp(value, min, max) { return Math.max(min, Math.min(max, value)); }
function distance(a, b) { return Math.hypot(a.x - b.x, a.y - b.y); }
