# EverArcade Public Demonstration Visual Specification

## Visual Identity

The public demo should look like infrastructure for persistent worlds: dark, futuristic, operator-oriented, and world-centric. It should combine the clarity of GitHub, Docker, and datacenter dashboards with the scope of a civilizational MMO.

Avoid:

- generic fantasy MMO styling,
- generic crypto UI tropes,
- casino-like glow overload,
- explosion-driven trailer effects,
- particle spam.

Target:

- deployable artifact clarity,
- operational trust,
- deterministic state visibility,
- portable world continuity.

## Color Palette

| Role | Color | Usage |
| --- | --- | --- |
| Background | Near Black | Primary app background and panel surfaces |
| Primary Accent | Electric Yellow | World artifact, command focus, final reveal |
| Secondary | Neon Cyan | Data flow, package transfer, operator links |
| Success | Bright Green | PASS states, root match, online operators |
| Warning | Amber | pending checks, migration in progress |
| Error | Red | failed checks only; avoid during PASS demo unless showing contrast |

## Layout

### Left Panel: World Overview

Display live values:

```text
World Name
World Hash
State Root
Tick
Epoch
Receipt Count
```

Treatment:

- Keep the panel pinned throughout the demo.
- Use electric yellow for the active world artifact label.
- Use monospaced text for hashes, roots, ticks, epochs, and receipt counts.
- Animate value changes with subtle one-frame cyan flashes.

### Center Panel: Projection Runtime

Render minimalist, readable state representations:

```text
players
entities
combat
inventory
market
governance
```

Treatment:

- Players are small labeled nodes, not character art.
- Entities are geometric markers with clear state tags.
- Combat is shown as intent, resolution, and receipt emission.
- Inventory changes are displayed as deltas.
- Market trades are displayed as signed exchanges.
- Governance updates are displayed as proposal and vote state changes.

The center panel should communicate that gameplay is a projection of state, not that state is a side effect of gameplay.

### Right Panel: Operator View

Display:

```text
Operator A
Operator B
Operator C
```

Each operator card must show:

```text
state root
world hash
status
```

Treatment:

- Green status means the operator world hash and state root match the active world.
- Amber status means syncing or migration in progress.
- Red status is reserved for mismatch or failure states.
- Root matching should animate as a subtle cyan line convergence into a green verification bar.

## Certification Overlay

Always visible, small, and unobtrusive:

```text
Tier 1: PASS
Tier 2: PASS
Projection Runtime: PASS
World Package: PASS
```

Treatment:

- Position in the lower-left or lower-right safe area.
- Use compact green PASS tags.
- Do not obscure the World Overview or Operator View.

## Motion Design

Use subtle infrastructure-grade animations:

- root synchronization,
- migration transfer,
- receipt flow,
- operator status transitions,
- certificate attachment to `world.evr`,
- final capability ladder reveal.

Avoid:

- camera shake,
- explosive transitions,
- excessive bloom,
- particle spam,
- fast game-trailer cuts.

## Scene-Specific Visual Requirements

### Build World

- Terminal command is legible.
- `world.evr` appears as the primary artifact.
- Package structure expands beside the artifact.
- World Hash and State Root initialize in the left panel.

### Certify World

- Certification checks appear as stacked rows.
- Each PASS turns bright green.
- A certificate badge attaches to the world package.

### Deploy World

- Operator A, Operator B, and Operator C come online in sequence.
- The same World Hash propagates to all three cards.
- Each card turns green only after matching the package.

### Live World

- Projection runtime shows movement, combat, inventory, market, and governance updates.
- Tick, Epoch, State Root, and Receipt Count update live.
- Receipts flow from the runtime to the world overview and operators.

### Verification

- Operator roots enlarge briefly.
- Identical roots align vertically.
- `ROOTS MATCH` appears in green.

### Replay

- Live runtime dims.
- Receipt history scrubs forward.
- End root equals live root.
- `REPLAY VERIFIED` appears in green.

### Restore

- A checkpoint card is selected.
- The tick resumes from the checkpoint instead of resetting.
- `RESTORE VERIFIED` appears in green.

### Migration

- Operator A exports a Migration Package.
- Operator D receives the package.
- World Hash remains unchanged.
- State Root remains identical at migration boundary.
- `MIGRATION VERIFIED` appears in green.

### Final Reveal

Display capability ladder:

```text
CREATE
CERTIFY
VERIFY
DEPLOY
REPLAY
RESTORE
MIGRATE
CONTINUE
```

Then reveal:

```text
BUILD WORLDS.
NOT JUST GAMES.
```

Final frame:

```text
PUBLIC DEMONSTRATION CERTIFICATION: PASS
```

## Sound Design

Sound is optional. If used, keep it minimal:

- soft confirmation tone for PASS,
- deployment tone when operators come online,
- migration tone during package transfer,
- verification tone when roots match.

Avoid game trailer music, dramatic combat audio, hype stingers, and casino-like reward sounds.
