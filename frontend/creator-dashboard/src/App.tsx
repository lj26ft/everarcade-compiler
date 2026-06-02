import { addRustrig, createProject, deployGame, packageGame, rehearseGame, status } from "@everarcade/shared-api";
import { StatusBadge } from "@everarcade/shared-ui";

export const templates = ["Arena", "Action RPG", "RTS", "Civilization", "Survival", "Dungeon Crawler", "Sandbox", "MMO Prototype"];
export const rustrigCategories = ["Combat", "Inventory", "Quests", "Dialogue", "Economy", "World", "Crafting", "Progression", "Movement", "Interaction", "Factions", "UI"];
export const worldBuilderViews = ["Hierarchy", "Viewport", "Inspector", "Assets", "Simulation", "Replay"];
export const worldActions = ["Place Entity", "Move Entity", "Delete Entity", "Duplicate Entity", "Run Simulation", "Save World"];
export const gameplaySurfaces = ["Visual Logic", "Quest Builder", "Dialogue Builder", "Combat Rules", "Inventory Rules", "Progression Rules"];
export const packageDeployActions = ["Package Game", "Run Rehearsal", "Deploy Dry Run", "View Package", "View Hashes", "View Reports"];

export function App() {
  return <main>
    <h1>🎮 EverArcade Creator Dashboard</h1>
    <section aria-label="Creator Home">
      <h2>Arena Vanguard</h2>
      <button data-command="everarcade new">New Project</button>
      <button>Import Project</button>
      {["Projects", "Templates", "Rustrigs", "Deployments", "Reports"].map(section => <article key={section}>{section}</article>)}
    </section>
    <section aria-label="New Project Flow">
      <h2>Create Project → Choose Template → Generate Runtime Files → Open Project</h2>
      {templates.map(template => <button key={template} data-facade-command="everarcade new">{template}</button>)}
    </section>
    <section aria-label="Rustrig Browser">
      <h2>Rustrig Browser</h2>
      {rustrigCategories.map(category => <button key={category} data-rustrig-category={category}>{category}</button>)}
      {["Browse", "Search", "Install", "Remove", "Inspect", "Compose"].map(action => <button key={action} data-facade-command="everarcade add-rustrig">{action}</button>)}
    </section>
    <section aria-label="World Builder">
      <h2>🌍 World Builder</h2>
      {worldBuilderViews.map(view => <article key={view}>{view}</article>)}
      {worldActions.map(action => <button key={action}>{action}</button>)}
    </section>
    <section aria-label="Gameplay Builder">
      <h2>Executable Rustrig Gameplay Builder</h2>
      {gameplaySurfaces.map(surface => <article key={surface} data-runtime-authority="rustrig">{surface}</article>)}
    </section>
    <section aria-label="Package Deploy">
      <h2><StatusBadge state="package" /> & <StatusBadge state="deploy" /></h2>
      {packageDeployActions.map(action => <button key={action}>{action}</button>)}
    </section>
    <section aria-label="Deployment Dashboard">
      {['Deployment Status', 'Package Status', 'Runtime Status', 'Federation Status', 'Replay Status'].map(item => <article key={item}>{item}</article>)}
    </section>
  </main>;
}

export const creatorFacadeBindings = { createProject, addRustrig, packageGame, rehearseGame, deployGame, status };
