import { createServer } from "node:http";
import { randomUUID, createHash } from "node:crypto";

const runtimeHost = {
  sessionId: "arena-vanguard-live",
  runtimeHealth: "healthy",
  gatewayHealth: "healthy",
  tick: 0,
  replay: [],
  checkpoints: [],
  sessions: new Map(),
  enemies: new Map([["enemy-raider-1", { enemyId: "enemy-raider-1", x: 8, y: 0, zone: "Combat Area", health: 100, status: "hostile" }]]),
  metrics: { joinRate: 0, reconnectRate: 0, actionThroughput: 0, gatewayLatencyMs: 1, sessionDurationTicks: 0 }
};

function resumeToken(sessionId, playerId) {
  return `resume:${createHash("sha256").update(sessionId).update(playerId).digest("hex")}`;
}

function zoneFor(x, y) {
  if (x === 0 && y === 0) return "Spawn Area";
  if (Math.abs(x) <= 2 && Math.abs(y) <= 2) return "Safe Area";
  if (x >= 5) return "Combat Area";
  return "Loot Area";
}

function runtimeRecord(kind) {
  runtimeHost.tick += 1;
  runtimeHost.metrics.sessionDurationTicks = runtimeHost.tick;
  runtimeHost.replay.push(`${runtimeHost.tick}:${runtimeHost.sessionId}:${kind}`);
}

function persistCheckpoint() {
  runtimeHost.runtimeHealth = "checkpointing";
  runtimeHost.checkpoints.push(runtimeHost.replay.length);
  runtimeHost.runtimeHealth = "healthy";
}

function playerFor(body = {}) {
  const seed = body.playerSeed ?? body.playerId?.replace(/^player-/, "") ?? "guest";
  const playerId = `player-${seed}`;
  const existing = runtimeHost.sessions.get(playerId);
  if (existing) return existing;
  return {
    playerId,
    sessionId: runtimeHost.sessionId,
    characterId: `character-${seed}`,
    position: { x: 0, y: 0, zone: "Spawn Area" },
    health: 100,
    energy: 50,
    xp: 0,
    level: 1,
    inventory: ["starter blade"],
    connected: false,
    resumeToken: resumeToken(runtimeHost.sessionId, playerId)
  };
}

function worldStateFeed() {
  return {
    type: "WorldStateFeed",
    tick: runtimeHost.tick,
    sessionId: runtimeHost.sessionId,
    players: [...runtimeHost.sessions.values()],
    enemies: [...runtimeHost.enemies.values()],
    worldState: { zones: ["Spawn Area", "Combat Area", "Loot Area", "Safe Area"], authority: "runtime" },
    replaySize: runtimeHost.replay.length,
    checkpointAge: runtimeHost.replay.length - runtimeHost.checkpoints.length
  };
}

// Gateway handlers are transport-only: each handler submits to this runtime-owned reducer and returns runtime state.
function runtimeAction(type, body = {}) {
  const current = playerFor(body);
  if (type === "join") { runtimeHost.metrics.joinRate += 1; current.connected = true; }
  if (type === "leave") { current.connected = false; persistCheckpoint(); }
  if (type === "move") {
    current.position.x += Number(body.dx ?? 0);
    current.position.y += Number(body.dy ?? 0);
    current.position.zone = zoneFor(current.position.x, current.position.y);
    current.energy = Math.max(0, current.energy - 1);
    runtimeHost.metrics.actionThroughput += 1;
  }
  if (type === "attack") {
    const enemy = runtimeHost.enemies.get(body.targetId ?? body.enemyId ?? "enemy-raider-1");
    if (enemy) {
      enemy.health = Math.max(0, enemy.health - 25);
      enemy.status = enemy.health === 0 ? "defeated" : "hostile";
      current.xp += 35;
      if (current.xp >= current.level * 100) { current.xp -= current.level * 100; current.level += 1; }
    }
    runtimeHost.metrics.actionThroughput += 1;
  }
  if (type === "interact") { current.inventory.push(body.item ?? "field cache"); current.xp += 10; runtimeHost.metrics.actionThroughput += 1; }
  if (type === "use-item") {
    const index = current.inventory.indexOf(body.item ?? body.itemId ?? "health potion");
    if (index >= 0) { current.inventory.splice(index, 1); current.health = Math.min(100, current.health + 20); }
    runtimeHost.metrics.actionThroughput += 1;
  }
  runtimeHost.sessions.set(current.playerId, current);
  runtimeRecord(type);
  return { status: "submitted", authority: "runtime", actionId: randomUUID(), session: current, feed: worldStateFeed() };
}

function resume(body = {}) {
  const session = [...runtimeHost.sessions.values()].find(candidate => candidate.resumeToken === body.resumeToken);
  if (!session) return { status: "failed", error: "invalid resume token" };
  session.connected = true;
  runtimeHost.metrics.reconnectRate += 1;
  runtimeRecord("resume");
  return { status: "resumed", authority: "runtime", session, feed: worldStateFeed() };
}

function heartbeat() {
  return { status: "ok", heartbeat: "valid", gatewayHealth: runtimeHost.gatewayHealth, runtimeHealth: runtimeHost.runtimeHealth, timeoutMs: 30000 };
}

function status() {
  const playerCount = [...runtimeHost.sessions.values()].filter(session => session.connected).length;
  return {
    status: "healthy",
    gatewayHealth: runtimeHost.gatewayHealth,
    runtimeHealth: runtimeHost.runtimeHealth,
    activeSessions: 1,
    sessionRegistry: [{ sessionId: runtimeHost.sessionId, playerCount, runtimeHealth: runtimeHost.runtimeHealth, checkpointAge: runtimeHost.replay.length - runtimeHost.checkpoints.length, replaySize: runtimeHost.replay.length }],
    playerCount,
    runtimeTick: runtimeHost.tick,
    replayGrowth: runtimeHost.replay.length,
    checkpointAge: runtimeHost.replay.length - runtimeHost.checkpoints.length,
    recoveryState: runtimeHost.runtimeHealth === "recovering" ? "recovering" : "stable",
    metrics: runtimeHost.metrics,
    feed: worldStateFeed()
  };
}

function readBody(request) {
  return new Promise(resolve => {
    let data = "";
    request.on("data", chunk => { data += chunk; });
    request.on("end", () => {
      try { resolve(data ? JSON.parse(data) : {}); } catch { resolve({}); }
    });
  });
}

const handlers = {
  "/join": body => runtimeAction("join", body),
  "/leave": body => runtimeAction("leave", body),
  "/move": body => runtimeAction("move", body),
  "/attack": body => runtimeAction("attack", body),
  "/interact": body => runtimeAction("interact", body),
  "/use-item": body => runtimeAction("use-item", body),
  "/resume": body => resume(body),
  "/heartbeat": () => heartbeat(),
  "/world-state": () => worldStateFeed(),
  "/status": () => status()
};

export const server = createServer(async (request, response) => {
  const url = new URL(request.url ?? "/", "http://localhost");
  const handler = handlers[url.pathname];
  if (!handler) {
    response.writeHead(404, { "content-type": "application/json" });
    response.end(JSON.stringify({ status: "failed", error: "unknown arena vanguard endpoint" }));
    return;
  }
  const body = await readBody(request);
  response.writeHead(200, { "content-type": "application/json" });
  response.end(JSON.stringify(handler(body)));
});

if (import.meta.url === `file://${process.argv[1]}`) {
  server.listen(Number(process.env.PORT ?? 8791), () => console.log("arena-vanguard-gateway live runtime binding listening"));
}
