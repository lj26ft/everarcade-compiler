import { connectWorld } from './world-adapter.js';

const canvas = document.querySelector('#game');
const ctx = canvas.getContext('2d');
const hud = document.querySelector('#hud');
const memory = document.querySelector('#memory');
const world = connectWorld();
let projection = world.readProjection();
const keys = new Set();

window.addEventListener('keydown', (event) => {
  keys.add(event.key.toLowerCase());
  if (event.code === 'Space') projection = world.submitInput({ type: 'attack' });
  if (event.key.toLowerCase() === 'e') projection = world.submitInput({ type: 'pickup' });
  if (event.key.toLowerCase() === 'r') projection = world.submitInput({ type: 'restore' });
  if (event.key.toLowerCase() === 'm') projection = world.submitInput({ type: 'migrate' });
  if (event.key.toLowerCase() === 'c') projection = world.submitInput({ type: 'open_chest' });
});
window.addEventListener('keyup', (event) => keys.delete(event.key.toLowerCase()));
canvas.addEventListener('click', () => { projection = world.submitInput({ type: 'attack' }); });

function step() {
  let dx = 0; let dy = 0;
  if (keys.has('arrowleft') || keys.has('a')) dx -= 8;
  if (keys.has('arrowright') || keys.has('d')) dx += 8;
  if (keys.has('arrowup') || keys.has('w')) dy -= 8;
  if (keys.has('arrowdown') || keys.has('s')) dy += 8;
  if (dx || dy) projection = world.submitInput({ type: 'move', dx, dy });
  draw(projection);
  requestAnimationFrame(step);
}

function draw(p) {
  ctx.fillStyle = '#10131c'; ctx.fillRect(0, 0, canvas.width, canvas.height);
  ctx.strokeStyle = '#2b3245'; ctx.lineWidth = 2;
  for (let x = 40; x < canvas.width; x += 80) for (let y = 40; y < canvas.height; y += 80) ctx.strokeRect(x, y, 80, 80);
  ctx.fillStyle = '#293047'; ctx.fillRect(680, 88, 72, 52); ctx.fillStyle = '#f2b84b'; ctx.fillRect(704, 108, 24, 18);
  for (const item of p.loot.filter((loot) => !loot.owner)) drawDiamond(item.x, item.y, '#5dff9c');
  for (const enemy of p.enemies) if (enemy.alive) { drawCircle(enemy.x, enemy.y, 18, '#b85cff'); bar(enemy.x - 22, enemy.y - 34, 44, 6, enemy.hp / (enemy.id.startsWith('warden') ? 12 : 8)); }
  drawCircle(p.player.x, p.player.y, 20, '#65d6ff'); bar(p.player.x - 24, p.player.y - 36, 48, 6, p.player.hp / 30);
  ctx.fillStyle = '#eaf7ff'; ctx.font = '14px monospace'; ctx.fillText('C chest · R restore · M migrate', 24, 30);
  hud.innerHTML = `<dl><dt>Player HP</dt><dd>${p.player.hp}</dd><dt>Inventory</dt><dd>${p.inventory.join(', ') || 'empty'}</dd><dt>Receipts</dt><dd>${p.receipts.count}</dd><dt>State root</dt><dd>${p.roots.state}</dd><dt>Receipt root</dt><dd>${p.roots.receipts}</dd><dt>Verify</dt><dd>${world.verifyWorld().status}</dd></dl>`;
  memory.innerHTML = p.memory.map((entry) => `<li>${entry}</li>`).join('');
}
function drawCircle(x, y, r, fill) { ctx.fillStyle = fill; ctx.beginPath(); ctx.arc(x, y, r, 0, Math.PI * 2); ctx.fill(); }
function drawDiamond(x, y, fill) { ctx.fillStyle = fill; ctx.beginPath(); ctx.moveTo(x, y - 16); ctx.lineTo(x + 16, y); ctx.lineTo(x, y + 16); ctx.lineTo(x - 16, y); ctx.closePath(); ctx.fill(); }
function bar(x, y, w, h, ratio) { ctx.fillStyle = '#351b24'; ctx.fillRect(x, y, w, h); ctx.fillStyle = '#ff647c'; ctx.fillRect(x, y, w * ratio, h); }

step();
