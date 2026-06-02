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
  enemies: new Map([["enemy-raider-1", { enemyId: "enemy-raider-1", position: { x: 8, y: 0, zone: "Combat Area" }, x: 8, y: 0, zone: "Combat Area", health: 100, status: "hostile", respawnTick: null }]]),
  loot: new Map([["loot-field-cache", { lootId: "loot-field-cache", itemId: "field cache", position: { x: 4, y: 0, zone: "Loot Area" }, available: true, claimedBy: null }]]),
  metrics: { joinRate: 0, reconnectRate: 0, actionThroughput: 0, gatewayLatencyMs: 1, sessionDurationTicks: 0, websocketCount: 0, connectionRate: 0, worldFeedRate: 0 }
};

const websocketClients = new Map();

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
    zones: ["Spawn Area", "Combat Area", "Loot Area", "Safe Area"],
    loot: [...runtimeHost.loot.values()],
    runtime_health: { runtime: runtimeHost.runtimeHealth, gateway: runtimeHost.gatewayHealth, recovery_state: runtimeHost.runtimeHealth === "recovering" ? "recovering" : "stable" },
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
      enemy.respawnTick = enemy.health === 0 ? runtimeHost.tick + 90 : null;
      current.xp += 35;
      if (current.xp >= current.level * 100) { current.xp -= current.level * 100; current.level += 1; }
    }
    runtimeHost.metrics.actionThroughput += 1;
  }
  if (type === "interact") {
    const item = body.item ?? body.itemId ?? "field cache";
    const loot = [...runtimeHost.loot.values()].find(candidate => candidate.itemId === item && candidate.available);
    if (loot) { loot.available = false; loot.claimedBy = current.playerId; current.inventory.push(item); current.xp += 10; }
    runtimeHost.metrics.actionThroughput += 1;
  }
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
  const connectedBrowsers = websocketClients.size;
  return {
    status: "healthy",
    gatewayHealth: runtimeHost.gatewayHealth,
    runtimeHealth: runtimeHost.runtimeHealth,
    activeSessions: 1,
    sessionRegistry: [{ sessionId: runtimeHost.sessionId, playerCount, runtimeHealth: runtimeHost.runtimeHealth, checkpointAge: runtimeHost.replay.length - runtimeHost.checkpoints.length, replaySize: runtimeHost.replay.length }],
    playerCount,
    connectedBrowsers,
    websocketConnections: runtimeHost.metrics.websocketCount,
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

function websocketAccept(key) {
  return createHash("sha1")
    .update(`${key}258EAFA5-E914-47DA-95CA-C5AB0DC85B11`)
    .digest("base64");
}

function encodeWebSocketFrame(value) {
  const payload = Buffer.from(JSON.stringify(value), "utf8");
  if (payload.length < 126) return Buffer.concat([Buffer.from([0x81, payload.length]), payload]);
  const header = Buffer.alloc(4);
  header[0] = 0x81;
  header[1] = 126;
  header.writeUInt16BE(payload.length, 2);
  return Buffer.concat([header, payload]);
}

function decodeWebSocketFrame(buffer) {
  const opcode = buffer[0] & 0x0f;
  if (opcode === 0x8) return { close: true };
  let offset = 2;
  let length = buffer[1] & 0x7f;
  if (length === 126) { length = buffer.readUInt16BE(offset); offset += 2; }
  if (length === 127) { length = Number(buffer.readBigUInt64BE(offset)); offset += 8; }
  const masked = (buffer[1] & 0x80) !== 0;
  const mask = masked ? buffer.subarray(offset, offset + 4) : undefined;
  if (masked) offset += 4;
  const payload = Buffer.from(buffer.subarray(offset, offset + length));
  if (mask) for (let index = 0; index < payload.length; index += 1) payload[index] ^= mask[index % 4];
  return { text: payload.toString("utf8") };
}

function websocketEventFor(message = {}) {
  const action = message.action ?? message;
  const type = action.type ?? message.type;
  if (type === "join") return { type: "PlayerJoin", payload: runtimeAction("join", action) };
  if (type === "leave") return { type: "PlayerLeave", payload: runtimeAction("leave", action) };
  if (type === "move") return { type: "PlayerUpdates", payload: runtimeAction("move", action) };
  if (type === "attack") return { type: "EnemyUpdates", payload: runtimeAction("attack", action) };
  if (type === "interact") return { type: "InventoryUpdates", payload: runtimeAction("interact", action) };
  if (type === "resume") return { type: "Resume", payload: resume(action) };
  if (type === "heartbeat") return { type: "Heartbeat", payload: heartbeat() };
  return { type: "WorldStateFeed", payload: worldStateFeed() };
}

function sendWebSocket(socket, value) {
  if (!socket.destroyed) socket.write(encodeWebSocketFrame(value));
}

server.on("upgrade", (request, socket) => {
  const url = new URL(request.url ?? "/", "http://localhost");
  if (url.pathname !== "/runtime-feed") {
    socket.destroy();
    return;
  }
  const key = request.headers["sec-websocket-key"];
  if (!key) {
    socket.destroy();
    return;
  }
  socket.write([
    "HTTP/1.1 101 Switching Protocols",
    "Upgrade: websocket",
    "Connection: Upgrade",
    `Sec-WebSocket-Accept: ${websocketAccept(key)}`,
    "",
    ""
  ].join("\r\n"));
  const connectionId = randomUUID();
  websocketClients.set(connectionId, socket);
  runtimeHost.metrics.websocketCount = websocketClients.size;
  runtimeHost.metrics.connectionRate += 1;
  sendWebSocket(socket, { sequence: 0, type: "WorldStateFeed", payload: worldStateFeed() });
  socket.on("data", frame => {
    const decoded = decodeWebSocketFrame(frame);
    if (decoded.close) { socket.end(); return; }
    const message = decoded.text ? JSON.parse(decoded.text) : {};
    const event = websocketEventFor(message);
    sendWebSocket(socket, { sequence: message.sequence ?? runtimeHost.tick, ...event });
  });
  socket.on("close", () => {
    websocketClients.delete(connectionId);
    runtimeHost.metrics.websocketCount = websocketClients.size;
  });
});

setInterval(() => {
  if (websocketClients.size === 0) return;
  runtimeHost.metrics.worldFeedRate += websocketClients.size;
  const event = { type: "WorldStateFeed", payload: worldStateFeed() };
  for (const socket of websocketClients.values()) sendWebSocket(socket, { sequence: runtimeHost.tick, ...event });
}, 250).unref();

if (import.meta.url === `file://${process.argv[1]}`) {
  server.listen(Number(process.env.PORT ?? 8791), () => console.log("arena-vanguard-gateway live runtime binding listening"));
}
