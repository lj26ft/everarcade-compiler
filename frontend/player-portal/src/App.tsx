import { status } from "@everarcade/shared-api";
import { connectWallet, disconnectWallet } from "@everarcade/shared-wallet";

export const playerHomeSections = ["Featured Games", "My Games", "My Profile", "Wallet", "Servers"];
export const gameBrowserColumns = ["Game Name", "Genre", "Players Online", "Version", "Status"];
export const arenaVanguardSections = ["Play", "Profile", "Inventory", "Progression", "Leaderboards", "World Status"];

export function App() {
  return <main>
    <h1>🎮 EverArcade Player Portal</h1>
    <section aria-label="Player Home">
      {playerHomeSections.map(section => <article key={section}>{section}</article>)}
    </section>
    <section aria-label="Featured Games">
      <h2>Arena Vanguard</h2>
      <p>First-class deterministic arena experience using runtime APIs only.</p>
    </section>
    <section aria-label="Game Browser">
      {gameBrowserColumns.map(column => <strong key={column}>{column}</strong>)}
      {['Join', 'Favorite', 'View Details'].map(action => <button key={action}>{action}</button>)}
    </section>
    <section aria-label="Arena Vanguard Portal" data-authority="runtime-read-only">
      {arenaVanguardSections.map(section => <article key={section}>{section}</article>)}
    </section>
    <section aria-label="Wallet">
      <button>Connect</button><button>Disconnect</button><span>Display Address</span><span>Display Network</span>
    </section>
  </main>;
}

export const playerPortalBindings = { connectWallet, disconnectWallet, status };
