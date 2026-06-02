import { createServer } from "node:http";
import { randomUUID } from "node:crypto";

const sessions = new Map();
let runtimeTick = 0;
let replayGrowth = 0;
let lastCheckpointTick = 0;

function runtimeAction(type, body = {}) {
  runtimeTick += 1;
  replayGrowth += 1;
  const seed = body.playerSeed ?? body.playerId ?? "guest";
  const playerId = `player-${seed}`;
  const sessionId = body.sessionId ?? `session-${seed}`;
  const current = sessions.get(playerId) ?? {
    playerId,
    sessionId,
    characterId: `character-${seed}`,
    x: 0,
    y: 0,
    health: 100,
    energy: 50,
    xp: 0,
    level: 1,
    inventory: [{ item: "gold", quantity: 0 }],
    connected: false
  };
  if (type === "join") current.connected = true;
  if (type === "leave") { current.connected = false; lastCheckpointTick = runtimeTick; }
  if (type === "move") { current.x += Number(body.dx ?? 0); current.y += Number(body.dy ?? 0); }
  if (type === "attack") { current.xp += 25; if (current.xp >= current.level * 100) { current.xp = 0; current.level += 1; } }
  if (type === "interact") current.inventory.push({ item: body.item ?? "health potion", quantity: Number(body.quantity ?? 1) });
  sessions.set(playerId, current);
  return { status: "submitted", authority: "runtime", runtimeTick, actionId: randomUUID(), session: current };
}

function status() {
  const playerCount = [...sessions.values()].filter(session => session.connected).length;
  return {
    status: "healthy",
    runtimeHealth: "healthy",
    worldHealth: "healthy",
    sessionCount: sessions.size,
    playerCount,
    runtimeTick,
    replayGrowth,
    checkpointAge: runtimeTick - lastCheckpointTick
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
  server.listen(Number(process.env.PORT ?? 8791), () => console.log("arena-vanguard-gateway listening"));
}
