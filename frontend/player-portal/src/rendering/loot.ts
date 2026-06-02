import type { WorldStateFeed } from "./types";

export function renderLoot(feed: WorldStateFeed) {
  return (feed.loot ?? []).filter(loot => loot.available).map(loot => ({ kind: "Loot", id: loot.lootId, item: loot.itemId, zone: loot.position.zone }));
}
