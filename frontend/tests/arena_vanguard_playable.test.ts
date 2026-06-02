export const arenaVanguardPlayableFrontendTests = [
  "Play Button",
  "Join Flow",
  "HUD Load",
  "Movement Input",
  "Combat Input",
  "Inventory View",
  "Reconnect Flow"
];

export function testPlayButton() { return { selector: "[data-testid=arena-vanguard-play]", action: "join" }; }
export function testJoinFlow() { return { flow: ["Click Play", "Join Session", "Spawn Character", "Load HUD", "Enter World"] }; }
export function testHudLoad() { return { hud: ["Health", "Energy", "XP", "Level", "Inventory", "Quest Tracker"], authority: "runtime-read-only" }; }
export function testMovementInput() { return { input: "Move", endpoint: "/move", authority: "runtime" }; }
export function testCombatInput() { return { input: "Attack", endpoint: "/attack", authority: "runtime" }; }
export function testInventoryView() { return { view: "Inventory", endpoint: "/interact", authority: "runtime-read-only" }; }
export function testReconnectFlow() { return { flow: ["leave", "join", "restore character", "restore inventory", "restore progression"] }; }
