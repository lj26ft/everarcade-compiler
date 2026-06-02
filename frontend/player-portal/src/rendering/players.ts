import type { WorldStateFeed } from "./types";

export function renderPlayers(feed: WorldStateFeed, ownPlayerId?: string) {
  return feed.players.map(player => ({
    kind: player.playerId === ownPlayerId ? "Own Player" : "Other Player",
    id: player.playerId,
    character: player.characterId,
    x: player.position.x,
    y: player.position.y,
    zone: player.position.zone,
    connected: player.connected
  }));
}
