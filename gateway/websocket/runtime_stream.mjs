import { createHash } from "node:crypto";

export const messageTypes = Object.freeze({
  join: "PlayerJoin",
  leave: "PlayerLeave",
  feed: "WorldStateFeed",
  heartbeat: "Heartbeat",
  resume: "Resume",
  action: "PlayerAction"
});

export function encodeFrame(sequence, type, payload = {}) {
  const envelope = { sequence, type, payload };
  return Buffer.from(JSON.stringify(envelope), "utf8");
}

export function decodeFrame(frame) {
  const text = Buffer.isBuffer(frame) ? frame.toString("utf8") : String(frame);
  return JSON.parse(text);
}

export function resumeToken(sessionId, playerId) {
  return `resume:${createHash("sha256").update(sessionId).update(playerId).digest("hex")}`;
}

export function deterministicConnectionId(playerSeed, ordinal = 0) {
  return `browser-${playerSeed}-${ordinal}`;
}

export function websocketReadiness() {
  return {
    status: "Ready",
    binarySafe: true,
    jsonSafe: true,
    deterministicOrdering: true,
    reconnectCapable: true,
    messages: Object.values(messageTypes)
  };
}
