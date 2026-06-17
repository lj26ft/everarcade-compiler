const POLL_MS = 1000;
const MAX_EVENTS = 120;
const state = { lastState: null, lastVerify: null, seenCombat: new Set(), combat: [], commitments: [] };

const $ = id => document.getElementById(id);
const canvas = $("arenaCanvas");
const ctx = canvas.getContext("2d");

async function fetchJson(path) {
  const response = await fetch(path, { cache: "no-store" });
  if (!response.ok) throw new Error(`${path} returned ${response.status}`);
  return response.json();
}

async function fetchAuthoritativeState() {
  const [world, verify] = await Promise.all([
    fetchJson("/state").catch(() => fetchJson("/world-state")),
    fetchJson("/verify").catch(() => fetchJson("/status"))
  ]);
  state.lastState = normalizeState(world);
  state.lastVerify = normalizeVerify(verify, state.lastState);
  rememberCommitment(state.lastState, state.lastVerify);
  render();
  setConnection(true);
}

function normalizeState(raw) {
  const feed = raw.feed ?? raw.worldStateFeed ?? raw;
  const players = Object.values(feed.players ?? raw.players ?? {}).map(player => ({
    id: player.playerId ?? player.id ?? player.player_id ?? "unknown-player",
    x: Number(player.position?.x ?? player.x ?? 0),
    y: Number(player.position?.y ?? player.y ?? 0),
    zone: player.position?.zone ?? player.zone ?? "Arena",
    health: Number(player.health ?? player.hp ?? 100),
    score: Number(player.score ?? player.xp ?? 0),
    connected: player.connected !== false && player.connectionStatus !== "disconnected"
  }));
  const journal = raw.journal ?? raw.replay ?? raw.entries ?? [];
  const combatEvents = [...(raw.combat_events ?? raw.combatEvents ?? []), ...journal.filter(entry => `${entry}`.includes("attack"))];
  for (const event of combatEvents) addCombat(event, feed.tick ?? raw.tick ?? 0);
  return {
    tick: Number(feed.tick ?? raw.tick ?? raw.runtimeTick ?? 0),
    players,
    enemies: Object.values(feed.enemies ?? raw.enemies ?? {}),
    journal,
    replayStatus: raw.replayStatus ?? raw.replay_status ?? feed.runtime_health?.recovery_state ?? "LIVE",
    journalSize: Number(raw.journalSize ?? raw.journal_size ?? feed.replaySize ?? journal.length ?? 0)
  };
}

function normalizeVerify(raw, normalized) {
  const roots = raw.roots ?? raw.commitment ?? raw.latestCommitment ?? raw.live ?? raw;
  const ok = raw.ok ?? raw.verified ?? raw.valid ?? ["verified", "healthy", "ok"].includes(String(raw.status ?? "").toLowerCase());
  return {
    verified: Boolean(ok),
    status: ok ? "VERIFIED" : "FAILED",
    stateRoot: roots.stateRoot ?? roots.state_root ?? roots.world_root ?? `state:arena-vanguard:tick:${normalized.tick}`,
    replayedStateRoot: raw.replayed?.stateRoot ?? raw.replayed?.state_root ?? null,
    receiptRoot: roots.receiptRoot ?? roots.receipt_root ?? roots.replay_root ?? `receipt:arena-vanguard:entries:${normalized.journalSize}`,
    replayedReceiptRoot: raw.replayed?.receiptRoot ?? raw.replayed?.receipt_root ?? null,
    worldHash: roots.worldHash ?? roots.world_hash ?? roots.checkpoint_root ?? `world:arena-vanguard:players:${normalized.players.length}`,
    replayedWorldHash: raw.replayed?.worldHash ?? raw.replayed?.world_hash ?? null,
    continuityRoot: roots.continuityRoot ?? roots.continuity_root ?? "continuity:arena-vanguard:live",
    replayedContinuityRoot: raw.replayed?.continuityRoot ?? raw.replayed?.continuity_root ?? null
  };
}

function addCombat(event, fallbackTick) {
  const text = typeof event === "string" ? event : (event.description ?? `${event.attacker ?? "player"} attacked ${event.target ?? "target"} for ${event.damage ?? 25} damage`);
  const tick = event.tick ?? fallbackTick;
  const key = `${tick}:${text}`;
  if (state.seenCombat.has(key)) return;
  state.seenCombat.add(key);
  state.combat.unshift({ tick, text, damage: event.damage ?? text.match(/(\d+) damage/)?.[1] ?? 25, at: new Date().toLocaleTimeString() });
  state.combat = state.combat.slice(0, MAX_EVENTS);
}

function rememberCommitment(world, verify) {
  const previous = state.commitments[0];
  const record = { tick: world.tick, ...verify };
  if (previous && previous.tick === record.tick && previous.stateRoot === record.stateRoot) return;
  state.commitments.unshift(record);
  state.commitments = state.commitments.slice(0, 80);
}

function setConnection(ok, error = "") {
  $("connectionDot").className = `dot ${ok ? "dot-ok" : "dot-fail"}`;
  $("connectionStatus").textContent = ok ? "Live runtime connected" : "Runtime unavailable";
  $("lastRefresh").textContent = ok ? `Updated ${new Date().toLocaleTimeString()}` : error;
}

function render() {
  const world = state.lastState;
  const verify = state.lastVerify;
  if (!world || !verify) return;
  drawArena(world);
  renderPlayers(world.players);
  renderRuntime(world, verify);
  renderVerification(verify);
  renderCombat();
  renderTimeline(world.journal);
  renderCommitments();
}

function drawArena(world) {
  const w = canvas.width, h = canvas.height, centerX = w / 2, centerY = h / 2, cell = 48;
  ctx.clearRect(0, 0, w, h);
  ctx.fillStyle = "#081827"; ctx.fillRect(0, 0, w, h);
  ctx.strokeStyle = "rgba(96,165,250,0.2)"; ctx.lineWidth = 1;
  for (let x = centerX % cell; x < w; x += cell) { ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, h); ctx.stroke(); }
  for (let y = centerY % cell; y < h; y += cell) { ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke(); }
  ctx.strokeStyle = "rgba(255,255,255,0.34)"; ctx.beginPath(); ctx.moveTo(centerX, 0); ctx.lineTo(centerX, h); ctx.moveTo(0, centerY); ctx.lineTo(w, centerY); ctx.stroke();
  for (const enemy of world.enemies) drawMarker(enemy.enemyId ?? enemy.id, enemy.position?.x ?? enemy.x ?? 0, enemy.position?.y ?? enemy.y ?? 0, enemy.health ?? 100, 0, true, "#ff5b6e");
  for (const player of world.players) drawMarker(player.id, player.x, player.y, player.health, player.score, player.connected, player.connected ? "#35e48a" : "#64748b");
}

function drawMarker(id, x, y, health, score, connected, color) {
  const px = canvas.width / 2 + x * 48;
  const py = canvas.height / 2 - y * 48;
  ctx.globalAlpha = connected ? 1 : 0.55;
  ctx.fillStyle = color; ctx.beginPath(); ctx.arc(px, py, 16, 0, Math.PI * 2); ctx.fill();
  ctx.strokeStyle = "#eef6ff"; ctx.lineWidth = 2; ctx.stroke();
  ctx.fillStyle = "#eef6ff"; ctx.font = "14px ui-sans-serif"; ctx.textAlign = "center";
  ctx.fillText(id, px, py - 26); ctx.fillText(`(${x}, ${y}) HP ${health} S ${score}`, px, py + 34);
  ctx.globalAlpha = 1;
}

function renderPlayers(players) {
  $("playerList").innerHTML = players.map(p => `<article class="player-card ${p.connected ? "" : "offline"}"><strong>${p.id}</strong><span>Position: (${p.x}, ${p.y}) · ${p.zone}</span><span>Health: ${p.health} · Score: ${p.score} · ${p.connected ? "Connected" : "Disconnected"}</span><div class="bars"><i style="width:${Math.max(0, Math.min(100, p.health))}%"></i></div></article>`).join("") || "<p class='subtitle'>No players have joined yet.</p>";
}

function renderRuntime(world, verify) {
  $("tickValue").textContent = world.tick;
  $("journalSize").textContent = world.journalSize;
  $("replayStatus").textContent = verify.status;
  $("playerCount").textContent = world.players.length;
}

function renderVerification(verify) {
  const badge = $("verifyBadge");
  badge.textContent = verify.verified ? "✓ VERIFIED" : "✗ FAILED";
  badge.className = verify.verified ? "verified" : "failed";
  const roots = [["State Root", verify.stateRoot], ["Receipt Root", verify.receiptRoot], ["World Hash", verify.worldHash], ["Continuity", verify.continuityRoot], ["Replayed State", verify.replayedStateRoot], ["Replayed Receipt", verify.replayedReceiptRoot], ["Replayed World", verify.replayedWorldHash], ["Replayed Continuity", verify.replayedContinuityRoot]].filter(([, value]) => value);
  $("roots").innerHTML = roots.map(([label, value]) => `<div class="root-row"><span>${label}</span><code title="${value}">${value}</code><button type="button" data-copy="${value}">Copy</button></div>`).join("");
}

function renderCombat() {
  $("combatLog").innerHTML = state.combat.map(e => `<li>${e.text}<small>tick ${e.tick} · damage ${e.damage} · ${e.at}</small></li>`).join("") || "<li>No attacks observed yet.</li>";
}

function renderTimeline(journal) {
  const entries = [...journal].slice(-MAX_EVENTS).reverse();
  $("timeline").innerHTML = entries.map((entry, index) => `<li>${formatJournal(entry)}<small>journal offset ${entries.length - index}</small></li>`).join("") || "<li>Waiting for Join, Move, and Attack entries.</li>";
}

function formatJournal(entry) {
  if (typeof entry === "string") return entry.split(":").pop()?.toUpperCase() ?? entry;
  return entry.type ?? entry.kind ?? JSON.stringify(entry);
}

function renderCommitments() {
  $("commitmentHistory").innerHTML = state.commitments.map(c => `<details><summary>Tick ${c.tick} · ${c.status}</summary><dl><div><dt>State Root</dt><dd>${c.stateRoot}</dd></div><div><dt>Receipt Root</dt><dd>${c.receiptRoot}</dd></div><div><dt>World Hash</dt><dd>${c.worldHash}</dd></div><div><dt>Continuity</dt><dd>${c.continuityRoot}</dd></div></dl></details>`).join("");
}

document.addEventListener("click", event => {
  const value = event.target?.dataset?.copy;
  if (!value) return;
  navigator.clipboard?.writeText(value);
  event.target.textContent = "Copied";
  setTimeout(() => { event.target.textContent = "Copy"; }, 900);
});

async function poll() {
  try { await fetchAuthoritativeState(); }
  catch (error) { setConnection(false, error.message); }
  finally { setTimeout(poll, POLL_MS); }
}

poll();
