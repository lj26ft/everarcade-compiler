# Platform Gap Analysis

## Prioritization Basis

Gaps are prioritized from the Arena Vanguard vertical-slice workflow. The priority order favors blockers to v0.1 confidence, then creator velocity, then polish.

| Area | Priority | Gap | Rationale |
| --- | --- | --- | --- |
| Deployment | P0 | Surface recovery drill status in the deployment wizard before publish approval. | v0.1 approval depends on real recovery and deployment confidence. |
| Editor | P1 | Ship a guided first-playable checklist inside Studio. | Reduces time to first playable for new developers. |
| Gameplay | P1 | Promote arena wave/loadout templates from sample data to reusable template assets. | Converts the official game workflow into reusable creator patterns. |
| Creator UX | P1 | Add deterministic package diff explanations for non-engine creators. | Helps creators understand reproducible output changes before publishing. |

## Non-goals

- Do not expand engine systems before validating them through the game workflow.
- Do not use manual runtime hacks to pass certification.
- Treat renderer, history, and federation domains as scaffold-level runtime domains unless a game-first gap proves otherwise.

## Recommended Next Work

1. Add the P0 deployment recovery gate to the Studio deployment wizard.
2. Build the guided first-playable checklist around Arena Vanguard.
3. Convert arena-specific content into reusable gameplay templates.
4. Add deterministic package diff explanations to the publishing flow.
