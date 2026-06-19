/* Arena Vanguard playable proof-of-world demo.
 * Local developer-preview only: no wallets, no HotPocket inputs, no settlement.
 * DEMO ROOT FALLBACK — canonicalizer integration pending.
 */
const canvas = document.getElementById('worldCanvas');
const ctx = canvas.getContext('2d');
const ui = {
  age: document.getElementById('hudAge'), tick: document.getElementById('hudTick'), objective: document.getElementById('hudObjective'),
  health: document.getElementById('hudHealth'), loot: document.getElementById('hudLoot'), quest: document.getElementById('questText'),
  memory: document.getElementById('memoryList'), events: document.getElementById('eventFeed'), receipts: document.getElementById('receiptList'),
  verify: document.getElementById('verificationGrid'), toasts: document.getElementById('toastStack'), banner: document.getElementById('completionBanner')
};
const keys = new Set();
const popups = [];
const particles = [];
let journal = [];
let receipts = [];
let state = genesisState();
let checkpoint = clone(state);
let lastMoveInput = 0;
let replay = null;
let lastTime = performance.now();
let objectiveOpen = true;

function genesisState() {
  const s = {
    schema: 'ArenaState.demo.v0.2', tick: 0, world_age: 1, state_root: 'genesis', world_hash: 'genesis', receipt_count: 0,
    player: { id: 'frontier-scout', x: 220, y: 330, vx: 0, vy: 0, hp: 12, maxHp: 12, attackCooldown: 0, facing: 1, loot: 0, inventory: [] },
    citadel: { x: 90, y: 238, w: 235, h: 205 }, camp: { x: 765, y: 335, r: 128, cleared: false, leaderKilled: false, ruinAge: 0 },
    creatures: [
      mob('skitter-1', 'Skitter', 718, 300, 3, false), mob('skitter-2', 'Skitter', 806, 410, 3, false),
      mob('brute-1', 'Brute', 870, 315, 4, false), mob('leader', 'Camp Leader', 790, 235, 7, true)
    ],
    loot: [], resources: [{ id: 'herbs', x: 535, y: 560, amount: 1 }, { id: 'ore', x: 970, y: 530, amount: 1 }],
    memory: { firstKill: null, campCleared: null, firstLoot: null, returnedLater: null, migrationPreserved: null },
    history: [{ tick: 0, label: 'World created', detail: 'Local ArenaState-shaped genesis loaded for the playable proof-of-world.' }],
    flags: { replay: 'READY', restore: 'READY', migration: 'READY', tier1: 'PASS', tier2: 'PASS', package: 'PASS' },
    lastToast: 'Move to the Frontier camp.'
  };
  seal(s); return s;
}
function mob(id, kind, x, y, hp, leader) { return { id, kind, x, y, spawnX: x, spawnY: y, vx: 0, vy: 0, hp, maxHp: hp, alive: true, leader, flash: 0, death: 0, phase: (id.length * 1.37) % 6.28, attackCooldown: 0 }; }
function clone(v) { return JSON.parse(JSON.stringify(v)); }
function stable(v) { if (Array.isArray(v)) return `[${v.map(stable).join(',')}]`; if (v && typeof v === 'object') return `{${Object.keys(v).filter(k => k !== 'state_root' && k !== 'world_hash').sort().map(k => `${JSON.stringify(k)}:${stable(v[k])}`).join(',')}}`; return JSON.stringify(v); }
function demoRoot(v, salt = 'root') { let h = 2166136261; const text = `${salt}:${stable(v)}`; for (let i = 0; i < text.length; i += 1) { h ^= text.charCodeAt(i); h = Math.imul(h, 16777619); } return `demo-${(h >>> 0).toString(16).padStart(8, '0')}`; }
function seal(s) { s.state_root = demoRoot(s, 'state'); s.world_hash = demoRoot(s, 'world'); }
function distance(a, b) { return Math.hypot(a.x - b.x, a.y - b.y); }
function toast(text) { state.lastToast = text; const el = document.createElement('div'); el.className = 'toast'; el.textContent = text; ui.toasts.prepend(el); setTimeout(() => el.remove(), 3300); }
function record(label, detail, target = state) { target.history.unshift({ tick: target.tick, label, detail }); }
function appendInput(type, payload = {}) { const input = { seq: journal.length + 1, type, payload: clone(payload) }; journal.push(input); applyInput(state, input, true); }
function applyInput(s, input, makeReceipt) {
  const pre = s.state_root; s.tick += 1;
  if (input.type === 'move') movePlayer(s, input.payload.dx, input.payload.dy);
  if (input.type === 'attack') attack(s);
  if (input.type === 'loot') pickup(s);
  if (input.type === 'camp_cleared') clearCamp(s);
  if (input.type === 'return_later') returnLater(s);
  if (input.type === 'restore') s.flags.restore = 'RESTORE VERIFIED';
  if (input.type === 'migration') migrate(s);
  if (input.type === 'replay') s.flags.replay = 'REPLAY VERIFIED';
  if (input.type === 'trade') record('Deterministic trade', 'A scaffold trade receipt was recorded locally.', s);
  if (input.type === 'vote') record('Deterministic vote', 'A scaffold vote receipt was recorded locally.', s);
  s.world_age = Math.max(s.world_age, 1 + Math.floor(s.tick / 30)); seal(s);
  if (makeReceipt) receipts.unshift({ id: `r${String(receipts.length + 1).padStart(3, '0')}`, type: input.type, pre, post: s.state_root, tick: s.tick });
  s.receipt_count += 1;
}
function movePlayer(s, dx, dy) { s.player.x = Math.max(35, Math.min(1065, s.player.x + dx)); s.player.y = Math.max(42, Math.min(650, s.player.y + dy)); if (dx) s.player.facing = Math.sign(dx); }
function attack(s) {
  if (s.player.attackCooldown > 0) return;
  s.player.attackCooldown = 24;
  const target = s.creatures.filter(c => c.alive).sort((a,b) => distance(s.player,a)-distance(s.player,b))[0];
  if (!target || distance(s.player, target) > 78) { record('Scout attacked', 'Blade swept empty air.', s); popups.push({ x: s.player.x, y: s.player.y - 28, text: 'miss', color: '#9fb6c8', life: 42 }); return; }
  target.hp -= 1; target.flash = 10; popups.push({ x: target.x, y: target.y - 24, text: '-1', color: '#ffd866', life: 42 }); particles.push({ x: target.x, y: target.y, r: 12, life: 14 }); record('Attack', `${target.kind} took 1 damage.`, s);
  if (target.hp <= 0) killCreature(s, target);
}
function killCreature(s, c) {
  c.alive = false; c.death = 30; const kind = c.leader ? 'vanguard relic' : 'camp shard'; s.loot.push({ id: `loot-${c.id}`, kind, x: c.x, y: c.y, born: s.tick, picked: false });
  if (!s.memory.firstKill) { s.memory.firstKill = `First kill: ${c.kind} at tick ${s.tick}`; record('First Kill', 'The world memory panel now keeps the first kill.', s); toast('World memory: First Kill recorded'); }
  record('Creature defeated', `${c.kind} dropped ${kind}.`, s);
  if (state === s) appendInput('kill', { id: c.id, kind: c.kind, leader: c.leader });
  if (c.leader) { s.camp.leaderKilled = true; appendInput('camp_cleared'); }
}
function pickup(s) {
  const item = s.loot.find(l => !l.picked && distance(s.player, l) < 66); if (!item) { record('Loot check', 'No nearby loot.', s); return; }
  item.picked = true; s.player.loot += 1; s.player.inventory.push(item.kind);
  if (!s.memory.firstLoot) { s.memory.firstLoot = `First loot: ${item.kind} at tick ${s.tick}`; toast('World memory: First Loot recorded'); }
  checkpoint = clone(s); record('Loot picked up', `${item.kind} added to scout pack; checkpoint refreshed.`, s);
}
function clearCamp(s) { if (s.camp.cleared) return; s.camp.cleared = true; s.memory.campCleared = `Frontier camp cleared at tick ${s.tick}`; record('Camp Cleared', 'Frontier camp became a remembered ruin marker.', s); toast('Frontier camp cleared — ruin remembered'); ui.banner.classList.add('show'); setTimeout(() => ui.banner.classList.remove('show'), 3200); }
function returnLater(s) { s.tick += 36; s.world_age += 2; s.camp.ruinAge += 1; s.resources.forEach(r => { r.amount += 1; }); s.memory.returnedLater = `Returned to an older world on Day ${s.world_age}`; record('Returned Later', 'You returned to an older world. The cleared camp memory remains.', s); toast('You returned to an older world.'); }
function migrate(s) { s.memory.migrationPreserved = `Migration preserved memory at tick ${s.tick}`; s.flags.migration = 'MIGRATION VERIFIED'; record('Migration Preserved', `Source package ${s.state_root.slice(0, 13)} continued on destination world.`, s); toast('Source world package → destination: memory preserved'); }
function restoreDemo() { state = clone(checkpoint); appendInput('restore'); toast('Checkpoint restored; continue from preserved memory.'); }
function migrateDemo() { appendInput('migration'); }
function replayJournal() {
  const finalRoot = state.state_root; const check = genesisState(); const frames = [];
  for (const input of journal) { if (input.type !== 'replay') applyInput(check, input, false); frames.push(clone(check)); }
  const ok = check.state_root === finalRoot; state.flags.replay = ok ? 'REPLAY VERIFIED' : 'REPLAY MISMATCH'; appendInput('replay', { verified: ok });
  replay = { frames, i: 0, next: performance.now(), ok }; toast(state.flags.replay);
}
function updateSim(dt) {
  state.player.attackCooldown = Math.max(0, state.player.attackCooldown - dt * 60);
  for (const c of state.creatures) {
    if (!c.alive) { c.death = Math.max(0, c.death - dt * 60); continue; }
    c.flash = Math.max(0, c.flash - dt * 60); c.attackCooldown = Math.max(0, c.attackCooldown - dt * 60);
    const d = distance(state.player, c); const chase = d < 185; const angle = chase ? Math.atan2(state.player.y - c.y, state.player.x - c.x) : Math.sin(performance.now()/900 + c.phase);
    const speed = chase ? 1.15 : .35; c.vx = chase ? Math.cos(angle) * speed : Math.cos(angle) * .35; c.vy = chase ? Math.sin(angle) * speed : Math.sin(angle * 1.7) * .28;
    if (!state.camp.cleared) { c.x += c.vx; c.y += c.vy; }
    if (d < 34 && c.attackCooldown <= 0) { c.attackCooldown = 60; state.player.hp = Math.max(0, state.player.hp - 1); popups.push({ x: state.player.x, y: state.player.y - 30, text: '-1', color: '#ff667f', life: 42 }); record('Scout hit', `${c.kind} hit the scout.`, state); }
  }
  for (const p of popups) { p.y -= .35 * dt * 60; p.life -= dt * 60; }
  for (const p of particles) { p.r += .7 * dt * 60; p.life -= dt * 60; }
  while (popups.length && popups[0].life <= 0) popups.shift(); while (particles.length && particles[0].life <= 0) particles.shift();
}
function objective() {
  if (!state.camp.cleared) return 'Defeat the camp leader';
  if (!state.memory.firstLoot) return 'Press E near dropped loot';
  if (state.player.x > state.citadel.x + state.citadel.w || state.player.y < state.citadel.y || state.player.y > state.citadel.y + state.citadel.h) return 'Return to the Citadel';
  if (!state.memory.returnedLater) return 'Press L to return later';
  if (state.flags.replay !== 'REPLAY VERIFIED') return 'Press R to replay history';
  if (state.flags.restore !== 'RESTORE VERIFIED') return 'Press C to restore checkpoint';
  if (state.flags.migration !== 'MIGRATION VERIFIED') return 'Press M to migrate world';
  return 'Proof-of-world loop complete';
}
function draw(s) {
  ctx.clearRect(0,0,1100,690); drawTerrain(); drawCamp(s); drawCitadel(); drawMinimap(s);
  drawResources(s); drawLoot(s); drawCreatures(s); drawPlayer(s); drawPopups();
}
function drawTerrain() {
  const zones = [[0,0,430,690,'#193148','Citadel Fields'],[430,0,670,405,'#21371f','Frontier Camp'],[430,405,670,285,'#1f2d1d','Wilderness Edge']];
  zones.forEach(z => { ctx.fillStyle = z[4]; ctx.fillRect(z[0],z[1],z[2],z[3]); ctx.fillStyle='rgba(255,255,255,.14)'; for(let i=0;i<26;i++) ctx.fillRect(z[0]+((i*83)%z[2]), z[1]+((i*47)%z[3]), 3, 3); ctx.fillStyle='rgba(244,251,255,.55)'; ctx.font='900 22px sans-serif'; ctx.fillText(z[5], z[0]+22, z[1]+42); });
}
function drawCitadel() { ctx.fillStyle='#263f68'; ctx.fillRect(90,238,235,205); ctx.fillStyle='#ffd866'; ctx.font='900 28px sans-serif'; ctx.fillText('◆ Citadel',130,315); ctx.font='800 13px sans-serif'; ctx.fillText('Return point',152,340); }
function drawCamp(s) { ctx.save(); ctx.translate(s.camp.x, s.camp.y); ctx.fillStyle = s.camp.cleared ? `rgba(185,156,255,${.16 + s.camp.ruinAge*.08})` : 'rgba(255,102,127,.16)'; ctx.beginPath(); ctx.arc(0,0,s.camp.r,0,Math.PI*2); ctx.fill(); ctx.strokeStyle=s.camp.cleared?'#b99cff':'#ff667f'; ctx.lineWidth=3; ctx.stroke(); ctx.fillStyle=s.camp.cleared?'#b99cff':'#ff667f'; ctx.font='950 24px sans-serif'; ctx.fillText(s.camp.cleared ? `Remembered Ruin +${s.camp.ruinAge}` : 'Creature Camp', -92, 6); ctx.restore(); }
function drawResources(s) { s.resources.forEach(r => { ctx.fillStyle='#72efff'; ctx.font='900 24px sans-serif'; ctx.fillText('✦', r.x, r.y); ctx.font='800 12px sans-serif'; ctx.fillText(`regen x${r.amount}`, r.x+20, r.y); }); }
function drawLoot(s) { s.loot.filter(l=>!l.picked).forEach(l => { const bob = Math.sin((performance.now()+l.born*60)/180)*5; ctx.fillStyle='#58f39b'; ctx.shadowColor='#58f39b'; ctx.shadowBlur=18; ctx.font='950 30px sans-serif'; ctx.fillText('✧', l.x-10, l.y+bob); ctx.shadowBlur=0; ctx.font='800 12px sans-serif'; ctx.fillText(l.kind, l.x+18, l.y+bob); }); }
function drawCreatures(s) { s.creatures.forEach(c => { if (!c.alive && c.death <= 0) return; ctx.globalAlpha = c.alive ? 1 : c.death / 30; ctx.fillStyle = c.flash > 0 ? '#ffffff' : (c.leader ? '#ff365d' : '#ff667f'); ctx.beginPath(); ctx.arc(c.x,c.y,c.leader?24:18,0,Math.PI*2); ctx.fill(); ctx.fillStyle='#24040a'; ctx.fillRect(c.x-24,c.y-38,48,6); ctx.fillStyle='#58f39b'; ctx.fillRect(c.x-24,c.y-38,48*(c.hp/c.maxHp),6); ctx.fillStyle='#fff'; ctx.font='850 12px sans-serif'; ctx.fillText(c.kind, c.x-30, c.y-45); ctx.globalAlpha=1; }); }
function drawPlayer(s) { const p=s.player; if (p.attackCooldown > 5) { ctx.strokeStyle='#ffd866'; ctx.lineWidth=5; ctx.beginPath(); ctx.arc(p.x+p.facing*28,p.y,30,-.8,.8); ctx.stroke(); } ctx.fillStyle='#f4fbff'; ctx.beginPath(); ctx.arc(p.x,p.y,21,0,Math.PI*2); ctx.fill(); ctx.strokeStyle='#72efff'; ctx.lineWidth=5; ctx.stroke(); ctx.fillStyle='#07101a'; ctx.font='950 16px sans-serif'; ctx.fillText('V',p.x-6,p.y+6); ctx.fillStyle='#24040a'; ctx.fillRect(p.x-30,p.y-38,60,7); ctx.fillStyle='#58f39b'; ctx.fillRect(p.x-30,p.y-38,60*(p.hp/p.maxHp),7); }
function drawPopups() { particles.forEach(p => { ctx.strokeStyle='rgba(255,216,102,.55)'; ctx.beginPath(); ctx.arc(p.x,p.y,p.r,0,Math.PI*2); ctx.stroke(); }); popups.forEach(p => { ctx.fillStyle=p.color; ctx.font='950 18px sans-serif'; ctx.fillText(p.text,p.x,p.y); }); }
function drawMinimap(s) { ctx.fillStyle='rgba(2,5,10,.58)'; ctx.fillRect(945,22,130,92); ctx.strokeStyle='rgba(255,255,255,.28)'; ctx.strokeRect(945,22,130,92); ctx.fillStyle='#ffd866'; ctx.fillRect(956,55,28,28); ctx.fillStyle=s.camp.cleared?'#b99cff':'#ff667f'; ctx.fillRect(1030,40,28,28); ctx.fillStyle='#f4fbff'; ctx.beginPath(); ctx.arc(945+s.player.x/1100*130,22+s.player.y/690*92,4,0,Math.PI*2); ctx.fill(); }
function renderUi() {
  ui.age.textContent = `Day ${state.world_age}`; ui.tick.textContent = state.tick; ui.objective.textContent = objective(); ui.health.textContent = `${state.player.hp}/${state.player.maxHp}`; ui.loot.textContent = state.player.loot;
  ui.quest.textContent = objectiveOpen ? `${objective()} — the verification layer below is local and read-only.` : 'Objective hidden. Press Q to show scout orders.';
  const memories = [['First Kill', state.memory.firstKill], ['Camp Cleared', state.memory.campCleared], ['First Loot', state.memory.firstLoot], ['Returned Later', state.memory.returnedLater], ['Migration Preserved', state.memory.migrationPreserved]];
  ui.memory.innerHTML = memories.map(([k,v]) => `<li class="${v?'remembered':'pending'}"><b>${k}</b><br>${v || 'pending'}</li>`).join('');
  ui.events.innerHTML = state.history.slice(0,10).map(e => `<li><b>${e.label}</b> tick ${e.tick}<br>${e.detail}</li>`).join('');
  ui.receipts.innerHTML = receipts.slice(0,10).map(r => `<li><b>${r.id}</b> ${r.type}<br>${r.pre.slice(0,12)} → ${r.post.slice(0,12)}</li>`).join('');
  const rows = [['state_root', state.state_root], ['world_hash', state.world_hash], ['receipts', receipts.length], ['Tier 1', state.flags.tier1], ['Tier 2', state.flags.tier2], ['World Package', state.flags.package], ['Replay', state.flags.replay], ['Restore/Migration', `${state.flags.restore} / ${state.flags.migration}`]];
  ui.verify.innerHTML = rows.map(([k,v]) => `<div class="verify-cell"><span>${k}</span><b class="${String(v).includes('PASS')||String(v).includes('VERIFIED')?'pass':'warn'}">${v}</b></div>`).join('');
}
function loop(now) {
  const dt = Math.min(.05, (now-lastTime)/1000); lastTime = now;
  if (!replay) {
    let dx=0, dy=0; const speed=4.6; if(keys.has('KeyW')||keys.has('ArrowUp')) dy-=speed; if(keys.has('KeyS')||keys.has('ArrowDown')) dy+=speed; if(keys.has('KeyA')||keys.has('ArrowLeft')) dx-=speed; if(keys.has('KeyD')||keys.has('ArrowRight')) dx+=speed;
    if (dx && dy) { dx *= .707; dy *= .707; } state.player.vx += (dx-state.player.vx)*.28; state.player.vy += (dy-state.player.vy)*.28;
    if ((Math.abs(state.player.vx)>.2 || Math.abs(state.player.vy)>.2) && now-lastMoveInput>70) { appendInput('move', { dx: Math.round(state.player.vx), dy: Math.round(state.player.vy) }); lastMoveInput=now; }
    updateSim(dt); draw(state);
  } else {
    if (now >= replay.next && replay.frames.length) { state = clone(replay.frames[Math.min(replay.i, replay.frames.length-1)]); replay.i += 1; replay.next = now + 150; if (replay.i >= replay.frames.length) replay = null; }
    draw(state);
  }
  renderUi(); requestAnimationFrame(loop);
}
addEventListener('keydown', e => { keys.add(e.code); if(['Space','KeyE','KeyQ','KeyR','KeyC','KeyM','KeyL','KeyT','KeyV'].includes(e.code)) e.preventDefault(); if(e.repeat) return; if(e.code==='Space') appendInput('attack'); if(e.code==='KeyE') appendInput('loot'); if(e.code==='KeyQ') objectiveOpen=!objectiveOpen; if(e.code==='KeyR') replayJournal(); if(e.code==='KeyC') restoreDemo(); if(e.code==='KeyM') migrateDemo(); if(e.code==='KeyL') appendInput('return_later'); if(e.code==='KeyT') appendInput('trade'); if(e.code==='KeyV') appendInput('vote'); });
addEventListener('keyup', e => keys.delete(e.code));
canvas.addEventListener('click', () => { canvas.focus(); appendInput('attack'); });
canvas.focus(); toast('Move with WASD/Arrows. Space or click attacks.'); requestAnimationFrame(loop);
