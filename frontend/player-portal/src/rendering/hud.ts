import type { WorldStateFeed } from "./types";

export function renderHud(feed: WorldStateFeed, playerId: string) {
  const player = feed.players.find(candidate => candidate.playerId === playerId);
  if (!player) return undefined;
  return {
    source: "WorldStateFeed",
    health: player.health,
    energy: player.energy,
    level: player.level,
    xp: player.xp,
    inventory: player.inventory,
    questStatus: player.connected ? "Session Active" : "Disconnected"
  };
}
