import type { WorldStateFeed } from "./types";

export function renderZones(feed: WorldStateFeed) {
  return (feed.zones ?? feed.world_zones ?? []).map(zone => ({ kind: "World Zone", name: zone }));
}
