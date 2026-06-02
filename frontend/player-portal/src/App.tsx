import { arenaVanguardGateway, status } from "@everarcade/shared-api";
import { connectWallet, disconnectWallet } from "@everarcade/shared-wallet";
import { renderEnemies, renderHud, renderLoot, renderPlayers, renderZones } from "./rendering";

export const playerHomeSections = ["Featured Games", "My Games", "My Profile", "Wallet", "Servers"];
export const gameBrowserColumns = ["Game Name", "Genre", "Players Online", "Version", "Status"];
export const arenaVanguardSections = ["Play", "Profile", "Inventory", "Progression", "Leaderboards", "World Status"];
export const arenaVanguardHud = ["Health", "Energy", "XP", "Level", "Inventory", "Quest Status"];
export const arenaVanguardWorldZones = ["Arena Outpost", "Spawn Area", "Combat Area", "Loot Area", "Safe Area"];
export const arenaVanguardRuntimeFlow = [
  "Click Play",
  "Join Session",
  "Spawn Character",
  "Load HUD",
  "Enter World",
  "Movement Input",
  "Combat Input",
  "Inventory View",
  "WorldStateFeed",
  "Reconnect Flow",
  "Session Resume",
  "Connect WebSocket",
  "Subscribe Feed",
  "Render Other Players",
  "Render Enemies",
  "Render Loot",
  "Runtime HUD Binding"
];

export function App() {
  return <main>
    <h1>🎮 EverArcade Player Portal</h1>
    <section aria-label="Player Home">
      {playerHomeSections.map(section => <article key={section}>{section}</article>)}
    </section>
    <section aria-label="Featured Games">
      <h2>Arena Vanguard</h2>
      <p>First-class deterministic arena experience using runtime APIs only.</p>
      <button data-testid="arena-vanguard-play" data-runtime-action="join" data-gateway="/join">Play Arena Vanguard</button>
    </section>
    <section aria-label="Game Browser">
      {gameBrowserColumns.map(column => <strong key={column}>{column}</strong>)}
      {["Join", "Favorite", "View Details"].map(action => <button key={action}>{action}</button>)}
    </section>
    <section aria-label="Arena Vanguard Portal" data-authority="runtime-read-only">
      {arenaVanguardSections.map(section => <article key={section}>{section}</article>)}
      <ol aria-label="Arena Vanguard Join Flow">
        {arenaVanguardRuntimeFlow.map(step => <li key={step}>{step}</li>)}
      </ol>
    </section>
    <section aria-label="Arena Vanguard HUD" data-authority="runtime-state-feed">
      {arenaVanguardHud.map(item => <output key={item}>{item}</output>)}
    </section>
    <section aria-label="Arena Outpost World View" data-authority="runtime-read-only" data-transport="WorldStateFeed">
      {arenaVanguardWorldZones.map(zone => <article key={zone}>{zone}</article>)}
      <button data-runtime-action="move" data-gateway="/move">Move</button>
      <button data-runtime-action="attack" data-gateway="/attack">Attack</button>
      <button data-runtime-action="interact" data-gateway="/interact">Interact / Loot</button>
      <button data-runtime-action="resume" data-gateway="/resume">Resume Session</button>
      <output data-testid="connected-browsers">Connected Browsers</output>
      <output data-testid="websocket-health">WebSocket Healthy</output>
    </section>
    <section aria-label="Wallet">
      <button>Connect</button><button>Disconnect</button><span>Display Address</span><span>Display Network</span>
    </section>
  </main>;
}

export const playerPortalBindings = { arenaVanguardGateway, connectWallet, disconnectWallet, status, renderPlayers, renderEnemies, renderLoot, renderZones, renderHud };
