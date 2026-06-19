# Arena Vanguard Public Demo Runbook

## Phase 1: recorded guided demo

Use seed or replay mode to control the story: live state, operator roots, replay, restore, migration, and verification.

## Phase 2: read-only live demo

Expose projection UI, operator dashboard, replay timeline, root details, and certification overlay. Do not expose mutation controls, wallet controls, settlement, governance authority, market authority, package deployment, signing, or admin controls.

## Modes

- `?mode=live` uses `/state` and `/verify` when available.
- `?mode=seed` uses `demo-world-seed.json`.
- `?mode=replay` presents the seed journal as replay markers.
- `?mode=migration` emphasizes the migration continuity flow.
- `?mode=restore` emphasizes checkpoint recovery.

## Local check

Serve the repository or projection directory on localhost, open `runtime/games/arena-vanguard/projection/dashboard.html?mode=seed`, confirm the arena renders, timeline markers animate visually, operator dashboard shows ROOTS MATCH, and the projection remains read-only.
