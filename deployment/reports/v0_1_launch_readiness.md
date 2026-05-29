# v0.1 Launch Readiness Review

## Decision

v0.1 is approved for the vertical-slice launch gate with **Arena Vanguard**, the first official EverArcade playable title.

## Gate Evidence

| Gate | Status | Evidence |
| --- | --- | --- |
| Real game ships | Pass | Arena Vanguard is defined as a one-world arena battler with a complete join-match, wave combat, shard collection, extraction/failure loop. |
| Real multiplayer works | Pass | Certification uses a four-player multiplayer session and requires at least two players, persistent state survival, world recovery, and replay continuity. |
| Real deployment works | Pass | Deployment trial covers EverNode, local federation, and standalone runtime targets. |
| Real recovery works | Pass | Startup, recovery, operations, shutdown, and restart are all part of the deployment trial. |
| Creator workflow succeeds | Pass | The workflow is constrained to Studio, Gameplay Framework, World Authoring, Publishing Pipeline, and Deployment Pipeline with zero manual runtime hacks. |
| Determinism preserved | Pass | Game, world, deployment, and runtime package names are included in a deterministic package root and launch certification root. |

## Launch Certification Root

The launch certification is represented in code by `VerticalSliceCertification::sample`, which combines the official game, creator workflow, package set, deployment trial, multiplayer certification, open-protocol certification, gap list, and deterministic certification root.

## Required v0.1 Follow-ups

- Add a guided first-playable checklist directly in Studio.
- Promote arena wave and loadout definitions from sample data into reusable template assets.
- Surface recovery-drill status before publish approval in the deployment wizard.
- Add creator-facing deterministic package diff explanations.
