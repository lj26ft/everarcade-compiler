# World Template Library v1

The World Template Library provides certified-by-construction starting points for EverArcade developers. Templates exist so a developer can begin with a meaningful world archetype instead of assembling every World Package directory, World Contract declaration, RustRig selection, genesis state, projection, continuity rule, and certification mapping from scratch.

Templates are not games. They are reusable world archetypes that developers customize into production worlds.

## Common Template Schema

Each template uses `world-template.toml` with these sections:

- `[template]`: template id, display name, version, description, and example world.
- `[world]`: world type and whether Continuity Engine behavior is enabled.
- `[rustrigs]`: enabled RustRig declarations.
- `[projection]`: default starter projection.
- `[certification]`: target certification level and mapping file.

Each template also includes `world-contract.toml`, `genesis/`, `continuity/`, `projection/`, `registry/`, `assets/`, and `README.md`.

## World Archetypes

| Template | Purpose | Demonstrates |
| --- | --- | --- |
| Arena | Fast PvE/PvP gameplay worlds | Combat-focused sessions, leaderboards, and loot |
| Frontier | Persistent survival and exploration worlds | The world remembers |
| Settlement | Economic and governance worlds | Worlds as economies |
| Social | Community and social worlds | Worlds as communities |
| Civilization | Full Continuity Engine worlds | Worlds as digital civilizations |

## Relationship to RustRigs

RustRigs are the certified runtime capabilities a template enables. A template narrows the RustRig surface area to match the archetype: Arena enables combat and loot actions, Frontier enables resources and structures, Settlement enables market and governance actions, Social enables community actions, and Civilization combines all major RustRig domains.

## Relationship to World Packages

A template is a starter World Package layout. Developers can copy the template, edit the generated World Contract, replace genesis state, customize assets, and ship the resulting package through the normal EverArcade build and deployment flow.

## Relationship to Certification

Certification follows this path:

```text
Template
→ World Contract
→ RustRig Set
→ World Package
→ Certification
→ Deployment
```

The template registry includes the initial certification mapping, expected receipts, authority rules, continuity behavior, and checks that should be satisfied before deployment.

## Relationship to the Continuity Engine

Continuity controls whether the world remembers long-lived state. Arena uses minimal continuity for match summaries and leaderboard deltas. Frontier, Settlement, Social, and Civilization demonstrate progressively richer continuity, from settlement aging and market history to institutional memory, migration, and civilization-scale timelines.

## Future CLI Behavior

```bash
everarcade world init --template arena
everarcade world init --template frontier
everarcade world init --template settlement
everarcade world init --template social
everarcade world init --template civilization
```

Example output:

```text
Created world package from template: frontier
Included RustRigs: identity.join, position.move, resource.harvest, structure.build, continuity.advance
Next steps: customize world-contract.toml, update assets, build package, deploy world
```

## Developer Flow

1. Choose template.
2. Customize contract.
3. Customize assets.
4. Build package.
5. Deploy world.
