export const liveSessionValidation = [
  "Play Flow",
  "Join Flow",
  "Movement",
  "Combat",
  "Inventory",
  "Reconnect",
  "HUD Sync",
  "Session Resume"
];

export function testPlayFlow() { return { click: "Play Arena Vanguard", gateway: "/join", runtime: "spawn" }; }
export function testJoinFlow() { return { flow: ["Click Play", "Gateway Join", "Runtime Spawn", "Live Session"] }; }
export function testMovement() { return { action: "Move", gateway: "/move", authority: "runtime" }; }
export function testCombat() { return { action: "Attack", gateway: "/attack", authority: "runtime" }; }
export function testInventory() { return { action: "Interact", gateway: "/interact", source: "WorldStateFeed" }; }
export function testReconnect() { return { disconnect: "/leave", reconnect: "/resume", restores: ["position", "inventory", "progression"] }; }
export function testHudSync() { return { source: "Runtime State Feed", fields: ["Health", "Energy", "XP", "Level", "Inventory", "Quest Status"] }; }
export function testSessionResume() { return { token: "resumeToken", heartbeat: "/heartbeat", state: "/world-state" }; }
