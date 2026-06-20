# EverArcade Open Source Launch Overview

EverArcade is an open-source platform for building, packaging, running, and verifying deterministic game worlds. It is designed for worlds whose rules, state transitions, continuity history, and recovery evidence can be inspected outside the original operator or platform.

The v0.1 launch package is focused on local developer proof: create a First World, run it, package it, verify it, and understand the evidence it emits. It is not a production network claim.

## What is EverArcade?

EverArcade combines creator tooling, templates, deterministic runtime paths, World Packages, and verification documentation into one repository. The repository helps developers answer:

- what rules define this world;
- what artifact should operators run;
- what evidence proves that execution followed the declared rules;
- how the world can be replayed, restored, or migrated without trusting private platform state.

## What is a World?

A **World** is a persistent game or simulation with declared rules, genesis state, continuity policy, and deterministic state transitions. A World can be an arena match, frontier settlement, social space, economy, civilization simulation, or another deterministic gameplay loop.

## What is a World Package?

A **World Package** is the portable artifact for a World. It carries the manifest, World Contract, RustRig declarations, genesis state, assets, continuity policy, proof mappings, and certification evidence needed for an operator or verifier to inspect and run the World.

In the Creator SDK flow, the creator-facing package is emitted as `dist/world.evr`.

## What is a World Contract?

A **World Contract** is the rule declaration for a World. It names active mutations, authorities, state domains, invariants, and RustRig bindings. The contract is the verifier-facing source of intent: runtime behavior should match what the contract declares.

## What is a RustRig?

A **RustRig** is a reusable deterministic gameplay mutation library. RustRigs cover domains such as combat, inventory, market, governance, identity, movement, resources, structures, crafting, quests, and continuity. A World Contract composes RustRigs into a world-specific rulebook.

## What is the Continuity Engine?

The **Continuity Engine** is EverArcade's model for preserving world history across replay, restore, checkpoint, and migration flows. It treats journals, receipts, state roots, checkpoints, and continuity policies as first-class evidence so a world can prove where it came from and resume from a verified point.

## Five-minute path

1. Read the repository README.
2. Run the First World flow.
3. Inspect the generated World Package.
4. Run verification and confirm `WORLD VERIFY: PASS`.
5. Pick a contributor path from the contributor guide.
