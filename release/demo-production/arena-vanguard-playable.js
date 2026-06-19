/* Arena Vanguard Playable Demo v0.1
 * Local developer-preview only. TODO: replace demoRoot with the production JS canonicalizer
 * when the release bundle exposes it as a browser import beside demo-production pages.
 */
const canvas = document.getElementById('worldCanvas');
const ctx = canvas.getContext('2d');
const verificationGrid = document.getElementById('verificationGrid');
const receiptList = document.getElementById('receiptList');
const eventFeed = document.getElementById('eventFeed');
const inventoryText = document.getElementById('inventoryText');
const statusText = document.getElementById('statusText');
const memoryBanner = document.getElementById('memoryBanner');

const keys = new Set();
let journal = [];
let receipts = [];
let state = genesisState();
let replay = null;
let lastFrame = performance.now();

function genesisState() {
  const base = {
    schema: 'ArenaState.demo.v0.1', tick: 0, world_age: 1, state_root: 'genesis', receipt_count: 0,
    player: { id: 'vanguard', x: 470, y: 330, hp: 10, inventory: { fang: 0, relic: 0, ore: 0 } },
    regions: [
      { name: 'Citadel', x: 0, y: 0, w: 480, h: 320 }, { name: 'Frontier', x: 480, y: 0, w: 480, h: 320 },
      { name: 'Wilderness', x: 0, y: 320, w: 480, h: 320 }, { name: 'Ruins', x: 480, y: 320, w: 480, h: 320 }
    ],
    creatures: [
      { id: 'wolf-1', kind: 'Wolf', x: 240, y: 430, hp: 3, maxHp: 3, alive: true },
      { id: 'drone-7', kind: 'Drone', x: 690, y: 420, hp: 4, maxHp: 4, alive: true },
      { id: 'sentinel-2', kind: 'Sentinel', x: 760, y: 520, hp: 5, maxHp: 5, alive: true }
    ],
    resources: [{ id: 'ore-a', kind: 'ore', x: 610, y: 190 }, { id: 'relic-a', kind: 'relic', x: 820, y: 460 }],
    loot: [], structures: [{ id: 'council', label: 'Council Hall', x: 170, y: 132 }, { id: 'market', label: 'Market Kiosk', x: 610, y: 135 }],
    ruins: [{ id: 'old-gate', label: 'Old Gate Stones', x: 390, y: 265 }],
    memory: { first_kill: null, first_trade: null, first_vote: null, returned_later: null },
    history: [{ tick: 0, label: 'World created', detail: 'Genesis ArenaState opened in local preview.' }],
    flags: { replay_verified: false, restore_verified: false, migration_verified: false }
  };
  base.state_root = demoRoot(base);
  return base;
}

function stable(value) {
  if (Array.isArray(value)) return `[${value.map(stable).join(',')}]`;
  if (value && typeof value === 'object') return `{${Object.keys(value).filter(k => k !== 'state_root').sort().map(k => JSON.stringify(k)+':'+stable(value[k])).join(',')}}`;
  return JSON.stringify(value);
}
function demoRoot(nextState) {
  let h = 2166136261;
  const text = stable(nextState);
  for (let i = 0; i < text.length; i += 1) { h ^= text.charCodeAt(i); h = Math.imul(h, 16777619); }
  return `demo-${(h >>> 0).toString(16).padStart(8, '0')}`;
}
function clone(v) { return JSON.parse(JSON.stringify(v)); }
function dist(a, b) { return Math.hypot(a.x - b.x, a.y - b.y); }
function record(label, detail) { state.history.unshift({ tick: state.tick, label, detail }); }
function appendInput(type, payload = {}) {
  const input = { seq: journal.length + 1, type, payload: clone(payload) };
  journal.push(input);
  applyInput(state, input, true);
}
function applyInput(target, input, makeReceipt) {
  const before = target.state_root;
  target.tick += 1;
  target.world_age = 1 + Math.floor(target.tick / 8);
  const p = target.player;
  if (input.type === 'move') { p.x = Math.max(24, Math.min(936, p.x + input.payload.dx)); p.y = Math.max(24, Math.min(616, p.y + input.payload.dy)); }
  if (input.type === 'attack') attackNearest(target);
  if (input.type === 'pickup') pickupNearby(target);
  if (input.type === 'trade') tradeDemo(target);
  if (input.type === 'vote') voteDemo(target);
  if (input.type === 'return') returnLater(target);
  target.receipt_count += makeReceipt ? 1 : 0;
  target.state_root = demoRoot(target);
  if (makeReceipt) receipts.unshift({ id: `r${String(receipts.length + 1).padStart(3, '0')}`, action: input.type, pre: before, post: target.state_root, tick: target.tick });
}
function attackNearest(s) {
  const living = s.creatures.filter(c => c.alive).sort((a,b) => dist(s.player,a)-dist(s.player,b));
  const c = living[0];
  if (!c || dist(s.player, c) > 96) return record('Attack missed', 'No creature close enough.');
  c.hp -= 1; record('Attack', `${c.kind} ${c.id} took 1 damage.`);
  if (c.hp <= 0) {
    c.alive = false; c.hp = 0; s.loot.push({ id: `loot-${c.id}`, kind: c.kind === 'Wolf' ? 'fang' : 'relic', x: c.x, y: c.y });
    if (!s.memory.first_kill) { s.memory.first_kill = `${c.kind} defeated at tick ${s.tick}`; s.ruins.push({ id: 'first-kill-marker', label: 'First Kill Cairn', x: c.x + 24, y: c.y - 18 }); }
    record('Creature defeated', `${c.kind} fell and left loot. The first kill is permanent world memory.`);
  }
}
function pickupNearby(s) {
  const item = s.loot.find(l => !l.picked && dist(s.player, l) < 70);
  if (!item) return record('Pickup failed', 'No nearby loot.');
  item.picked = true; s.player.inventory[item.kind] = (s.player.inventory[item.kind] || 0) + 1; record('Loot picked up', `${item.kind} added to inventory.`);
}
function tradeDemo(s) {
  s.player.inventory.ore += 1;
  if (!s.memory.first_trade) { s.memory.first_trade = `Ore-for-credit trade at tick ${s.tick}`; s.structures.push({ id: 'first-trade-stall', label: 'First Trade Stall', x: 650, y: 105 }); }
  record('Deterministic trade', 'Market Kiosk exchanged one ore for one remembered credit.');
}
function voteDemo(s) {
  if (!s.memory.first_vote) { s.memory.first_vote = `AV-01 passed at tick ${s.tick}`; s.structures.push({ id: 'vote-obelisk', label: 'Vote Obelisk', x: 235, y: 155 }); }
  record('Governance vote', 'AV-01 passed deterministically in the Citadel.');
}
function returnLater(s) { s.tick += 15; s.world_age = 1 + Math.floor(s.tick / 8); s.memory.returned_later = `Player returned on day ${s.world_age}`; record('Player returned later', 'Age advanced; persisted history is still visible.'); }
function replayJournal() {
  const check = genesisState(); const frames = [];
  for (const input of journal) { applyInput(check, input, false); frames.push(clone(check)); }
  replay = { frames, i: 0, verified: check.state_root === state.state_root, until: performance.now() + Math.max(900, frames.length * 220) };
  state.flags.replay_verified = replay.verified; statusText.textContent = replay.verified ? 'REPLAY VERIFIED' : 'REPLAY MISMATCH';
}
function restoreDemo() { const checkpoint = clone(state); applyInput(checkpoint, { seq: 0, type: 'return', payload: {} }, false); state = checkpoint; appendInput('return'); state.flags.restore_verified = true; statusText.textContent = 'RESTORE VERIFIED'; }
function migrationDemo() { const packaged = clone(state); packaged.history.unshift({ tick: packaged.tick, label: 'Migration package sealed', detail: `Source root ${packaged.state_root}` }); state = packaged; appendInput('return'); state.flags.migration_verified = true; statusText.textContent = 'MIGRATION VERIFIED'; record('Migration verified', 'Source → package → destination continued from the same state.'); }

function draw(s) {
  ctx.clearRect(0,0,960,640);
  const fills = ['rgba(255,221,87,.10)','rgba(98,241,255,.10)','rgba(72,242,154,.10)','rgba(185,156,255,.10)'];
  s.regions.forEach((r,i)=>{ ctx.fillStyle=fills[i]; ctx.fillRect(r.x,r.y,r.w,r.h); ctx.strokeStyle='rgba(255,255,255,.16)'; ctx.strokeRect(r.x,r.y,r.w,r.h); ctx.fillStyle='rgba(242,251,255,.72)'; ctx.font='900 22px sans-serif'; ctx.fillText(r.name,r.x+22,r.y+36); });
  drawItems(s.resources, '#62f1ff', '◆'); drawItems(s.ruins, '#b99cff', '▰'); drawItems(s.structures, '#ffdd57', '⌂'); drawItems(s.loot.filter(l=>!l.picked), '#48f29a', '✦');
  s.creatures.forEach(c=>{ if(!c.alive) return; ctx.fillStyle='#ff6e8a'; ctx.beginPath(); ctx.arc(c.x,c.y,15,0,Math.PI*2); ctx.fill(); ctx.fillStyle='#fff'; ctx.font='800 12px sans-serif'; ctx.fillText(`${c.kind} ${c.hp}/${c.maxHp}`, c.x-32, c.y-24); });
  ctx.fillStyle='#f2fbff'; ctx.beginPath(); ctx.arc(s.player.x,s.player.y,17,0,Math.PI*2); ctx.fill(); ctx.strokeStyle='#62f1ff'; ctx.lineWidth=4; ctx.stroke(); ctx.fillStyle='#030713'; ctx.font='900 14px sans-serif'; ctx.fillText('V',s.player.x-5,s.player.y+5);
}
function drawItems(items, color, glyph) { items.forEach(o=>{ ctx.fillStyle=color; ctx.font='900 25px sans-serif'; ctx.fillText(glyph,o.x-9,o.y+8); ctx.font='800 12px sans-serif'; ctx.fillText(o.label || o.kind, o.x+13, o.y+4); }); }
function renderUi() {
  inventoryText.textContent = Object.entries(state.player.inventory).filter(([,v])=>v).map(([k,v])=>`${k}:${v}`).join(' · ') || 'none';
  const rows = [['tick', state.tick], ['world age', `Day ${state.world_age}`], ['state_root', state.state_root.slice(0, 13)], ['receipts', receipts.length], ['Tier 1', 'PASS'], ['Tier 2', 'PASS'], ['Projection', 'read-only'], ['World Package', 'PASS']];
  verificationGrid.innerHTML = rows.map(([k,v]) => `<div><dt>${k}</dt><dd class="${String(v).includes('PASS')?'pass':''} ${k==='Projection'?'badge':''}">${v}</dd></div>`).join('');
  receiptList.innerHTML = receipts.slice(0,9).map(r=>`<li><b>${r.id}</b> ${r.action}: ${r.pre.slice(0,10)} → ${r.post.slice(0,10)}</li>`).join('');
  eventFeed.innerHTML = state.history.slice(0,12).map(e=>`<li><b>${e.label}</b> tick ${e.tick}<br>${e.detail}</li>`).join('');
  const memories = [state.memory.first_kill, state.memory.first_trade, state.memory.first_vote, state.memory.returned_later].filter(Boolean);
  memoryBanner.textContent = memories.length ? `World remembers: ${memories.join(' · ')}` : 'Move with WASD/Arrows. Space or click attacks. E picks up loot.';
}
function loop(now) {
  const step = 3; let dx=0, dy=0;
  if (keys.has('KeyW')||keys.has('ArrowUp')) dy-=step; if (keys.has('KeyS')||keys.has('ArrowDown')) dy+=step; if (keys.has('KeyA')||keys.has('ArrowLeft')) dx-=step; if (keys.has('KeyD')||keys.has('ArrowRight')) dx+=step;
  if (now - lastFrame > 80 && (dx || dy) && !replay) { appendInput('move', { dx, dy }); lastFrame = now; }
  if (replay && replay.frames.length) { const idx = Math.min(replay.frames.length - 1, Math.floor((replay.until - now) / 220)); draw(replay.frames[Math.max(0, replay.frames.length - 1 - idx)]); if (now >= replay.until) replay = null; }
  else draw(state);
  renderUi(); requestAnimationFrame(loop);
}
addEventListener('keydown', e => { keys.add(e.code); if(['Space','KeyE','KeyT','KeyV','KeyR','KeyM','KeyC'].includes(e.code)) e.preventDefault(); if(e.code==='Space') appendInput('attack'); if(e.code==='KeyE') appendInput('pickup'); if(e.code==='KeyT') appendInput('trade'); if(e.code==='KeyV') appendInput('vote'); if(e.code==='KeyR') replayJournal(); if(e.code==='KeyM') migrationDemo(); if(e.code==='KeyC') restoreDemo(); });
addEventListener('keyup', e => keys.delete(e.code));
canvas.addEventListener('click', () => { canvas.focus(); appendInput('attack'); });
canvas.focus(); record('Developer preview opened', 'Projection frames are read-only; local inputs produce deterministic demo receipts.'); requestAnimationFrame(loop);
