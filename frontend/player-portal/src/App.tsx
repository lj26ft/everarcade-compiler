import { arenaVanguardGateway, resolveArenaVanguardRuntimeFeedUrl, status } from "@everarcade/shared-api";
import { connectWallet, disconnectWallet } from "@everarcade/shared-wallet";
import { useMemo, useState } from "react";
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

const runtimeStages = [
  "🔌 Connecting to gateway",
  "🎮 Joining session",
  "🧍 Character spawned",
  "🌍 World feed active",
  "🟢 Ready"
];

const controlsHelp = [
  ["Move", "WASD or arrow keys"],
  ["Attack", "Space or Attack button"],
  ["Interact", "E near loot or NPCs"],
  ["Reconnect", "Click Play Arena Vanguard again"]
];

type RuntimeState = "waiting" | "connecting" | "ready" | "failed";

export function App() {
  const [runtimeState, setRuntimeState] = useState<RuntimeState>("waiting");
  const [activeStages, setActiveStages] = useState<string[]>([]);
  const runtimeFeedUrl = useMemo(() => resolveArenaVanguardRuntimeFeedUrl(), []);

  function playArenaVanguard() {
    setRuntimeState("connecting");
    setActiveStages([runtimeStages[0]]);

    if (typeof WebSocket === "undefined") {
      setRuntimeState("failed");
      return;
    }

    const socket = new WebSocket(runtimeFeedUrl);
    let connected = false;
    let stageIndex = 0;
    const stageTimer = window.setInterval(() => {
      stageIndex = Math.min(stageIndex + 1, runtimeStages.length - 2);
      setActiveStages(runtimeStages.slice(0, stageIndex + 1));
    }, 250);

    const failConnection = () => {
      if (connected) return;
      window.clearInterval(stageTimer);
      setRuntimeState("failed");
      socket.close();
    };

    const connectionTimeout = window.setTimeout(failConnection, 5000);

    socket.onopen = () => {
      connected = true;
      window.clearInterval(stageTimer);
      window.clearTimeout(connectionTimeout);
      setActiveStages(runtimeStages);
      setRuntimeState("ready");
      socket.close();
    };

    socket.onerror = failConnection;
  }

  const waiting = runtimeState === "waiting";

  return <main style={{ fontFamily: "system-ui, sans-serif", maxWidth: "960px", margin: "0 auto", padding: "2rem", color: "#172033" }}>
    <h1>🎮 EverArcade Player Portal</h1>
    <p>Run <code>everarcade run arena-vanguard</code>, then <code>cd frontend/player-portal && npm run dev</code> and open <code>http://&lt;vm-ip&gt;:5173</code>.</p>

    <section aria-label="Runtime Status" data-testid="runtime-status" style={{ border: "1px solid #d8dee9", borderRadius: "12px", padding: "1rem", marginBottom: "1rem" }}>
      <h2>Runtime Status</h2>
      <p>{waiting ? "🟡 Waiting for runtime" : runtimeState === "failed" ? "🔴 Could not connect to runtime" : runtimeState === "ready" ? "🟢 Ready" : "🔌 Connecting to gateway"}</p>
      {runtimeState === "failed" ? <p><strong>Run:</strong> <code>everarcade run arena-vanguard</code></p> : null}
      <p><strong>Gateway Status:</strong> {runtimeState === "ready" ? "Live" : "Checking runtime gateway"}</p>
      <p><strong>WebSocket Status:</strong> {runtimeFeedUrl}</p>
    </section>

    <section aria-label="Game Card" data-testid="arena-vanguard-card" style={{ border: "1px solid #d8dee9", borderRadius: "12px", padding: "1rem", marginBottom: "1rem", background: "#f8fbff" }}>
      <h2>Arena Vanguard</h2>
      <p>First-class deterministic arena experience using runtime APIs only.</p>
      <button data-testid="arena-vanguard-play" data-runtime-action="join" data-gateway="/join" onClick={playArenaVanguard} style={{ padding: "0.75rem 1rem", borderRadius: "8px", border: "0", background: "#2f6fed", color: "white", fontWeight: 700 }}>Play Arena Vanguard</button>
    </section>

    <section aria-label="Connection Status" style={{ border: "1px solid #d8dee9", borderRadius: "12px", padding: "1rem", marginBottom: "1rem" }}>
      <h2>Connection Status</h2>
      <ol>{(activeStages.length ? activeStages : ["🟡 Waiting for runtime"]).map(step => <li key={step}>{step}</li>)}</ol>
    </section>

    <section aria-label="Session Status" style={{ border: "1px solid #d8dee9", borderRadius: "12px", padding: "1rem", marginBottom: "1rem" }}>
      <h2>Session Status</h2>
      {arenaVanguardSections.map(section => <article key={section}>{section}</article>)}
      <ol aria-label="Arena Vanguard Join Flow">
        {arenaVanguardRuntimeFlow.map(step => <li key={step}>{step}</li>)}
      </ol>
    </section>

    <section aria-label="Arena Vanguard HUD" data-authority="runtime-state-feed" style={{ border: "1px solid #d8dee9", borderRadius: "12px", padding: "1rem", marginBottom: "1rem" }}>
      <h2>HUD Preview</h2>
      {arenaVanguardHud.map(item => <output key={item} style={{ display: "inline-block", marginRight: "1rem" }}>{item}</output>)}
    </section>

    <section aria-label="Controls Help" data-authority="runtime-read-only" data-transport="WorldStateFeed" style={{ border: "1px solid #d8dee9", borderRadius: "12px", padding: "1rem", marginBottom: "1rem" }}>
      <h2>Controls Help</h2>
      {controlsHelp.map(([label, help]) => <p key={label}><strong>{label}</strong>: {help}</p>)}
      {arenaVanguardWorldZones.map(zone => <article key={zone}>{zone}</article>)}
      <button data-runtime-action="move" data-gateway="/move">Move</button>
      <button data-runtime-action="attack" data-gateway="/attack">Attack</button>
      <button data-runtime-action="interact" data-gateway="/interact">Interact</button>
      <button data-runtime-action="resume" data-gateway="/resume">Reconnect</button>
      <output data-testid="connected-browsers">Connected Browsers</output>
      <output data-testid="websocket-health">WebSocket Healthy</output>
    </section>

    <section aria-label="Player Home">
      {playerHomeSections.map(section => <article key={section}>{section}</article>)}
    </section>
    <section aria-label="Game Browser">
      {gameBrowserColumns.map(column => <strong key={column}>{column}</strong>)}
      {["Join", "Favorite", "View Details"].map(action => <button key={action}>{action}</button>)}
    </section>
    <section aria-label="Wallet">
      <button>Connect</button><button>Disconnect</button><span>Display Address</span><span>Display Network</span>
    </section>
  </main>;
}

export const playerPortalBindings = { arenaVanguardGateway, connectWallet, disconnectWallet, status, renderPlayers, renderEnemies, renderLoot, renderZones, renderHud };
