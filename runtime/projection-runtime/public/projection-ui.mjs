const params = new URLSearchParams(location.search);
const mode = params.get('mode') ?? 'live';
const operator = params.get('operator') ?? 'Operator A';
const response = await fetch(params.get('journal') ?? '../../examples/projection-demo-world/journal.json');
const journal = await response.json();
async function digest(value) { const data = new TextEncoder().encode(JSON.stringify(sortKeys(value))); const hash = await crypto.subtle.digest('SHA-256', data); return [...new Uint8Array(hash)].map((b) => b.toString(16).padStart(2, '0')).join(''); }
function sortKeys(value) { if (Array.isArray(value)) return value.map(sortKeys); if (value && typeof value === 'object') return Object.fromEntries(Object.keys(value).sort().map((key) => [key, sortKeys(value[key])])); return value; }
async function projectArenaState(arenaState, options) { const state = structuredClone(arenaState); const receipts = state.receipts ?? []; return { projectionVersion: '0.1.0', operator: options.operator, mode: options.mode, worldName: state.world?.name ?? 'Unnamed Arena', epoch: state.continuity?.epoch ?? 0, tick: state.continuity?.tick ?? 0, root: await digest(state), players: (state.players ?? []).map((p) => ({ player_id: p.player_id, score: p.score ?? 0, status: p.status ?? 'unknown', inventory_count: (p.inventory ?? []).length })), entities: (state.entities ?? []).map((e) => ({ entity_id: e.entity_id, entity_type: e.entity_type })), positions: (state.positions ?? []).map((p) => ({ entity_id: p.entity_id, x: p.x, y: p.y })), health: (state.health ?? []).map((h) => ({ entity_id: h.entity_id, health: h.health, max_health: h.max_health })), latest_receipt_count: receipts.length, market_receipts: receipts.filter((r) => r.domain === 'market'), governance: (state.governance?.proposals ?? []).map((g) => ({ proposal_id: g.proposal_id, status: g.status, yes: g.yes ?? 0, no: g.no ?? 0 })) }; }
let index = 0;
const canvas = document.getElementById('world');
const ctx = canvas.getContext('2d');

function byId(id) { return document.getElementById(id); }
function list(id, items, render) { byId(id).innerHTML = items.map(render).join(''); }

function render(frame) {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = '#14223a';
  for (let x = 0; x < canvas.width; x += 48) for (let y = 0; y < canvas.height; y += 48) ctx.strokeRect(x, y, 48, 48);
  for (const pos of frame.positions) {
    const entity = frame.entities.find((candidate) => candidate.entity_id === pos.entity_id);
    const hp = frame.health.find((candidate) => candidate.entity_id === pos.entity_id);
    ctx.fillStyle = entity?.entity_type === 'player' ? '#50b4ff' : '#ff6e7a';
    ctx.beginPath(); ctx.arc(pos.x, pos.y, 16, 0, Math.PI * 2); ctx.fill();
    ctx.fillStyle = '#e6f3ff'; ctx.fillText(`${pos.entity_id} (${entity?.entity_type ?? 'entity'})`, pos.x + 20, pos.y);
    if (hp) { ctx.fillStyle = '#7dffa6'; ctx.fillRect(pos.x - 16, pos.y + 22, 32 * (hp.health / hp.max_health), 4); }
  }
  byId('worldName').textContent = frame.worldName;
  byId('mode').textContent = frame.mode;
  byId('operator').textContent = frame.operator;
  byId('epoch').textContent = frame.epoch;
  byId('tick').textContent = frame.tick;
  byId('root').textContent = frame.root;
  byId('receipts').textContent = frame.latest_receipt_count;
  list('players', frame.players, (p) => `<li>${p.player_id}: score ${p.score}, ${p.status}, items ${p.inventory_count}</li>`);
  list('market', frame.market_receipts, (r) => `<li>${r.receipt_id}: ${r.summary}</li>`);
  list('governance', frame.governance, (g) => `<li>${g.proposal_id}: ${g.status} (${g.yes}/${g.no})</li>`);
}

setInterval(async () => {
  const entry = journal.ticks[index];
  render(await projectArenaState(entry.arenaState, { mode, operator }));
  index = (index + 1) % journal.ticks.length;
}, 800);
