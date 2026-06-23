# Frontier Settlement RC1 Creator Guide

Frontier Settlement RC1 is a cooperative settlement world for creators who want to contribute economy, governance, lore, and verification work without learning compiler internals first.

## World premise

Founders Crossing is a small persistent settlement. Players gather and transfer materials, trade scarce resources through the market, and vote on settlement upgrades through council governance.

## Current deterministic loops

- `inventory.transfer`: move timber, ore, and charter items between players and settlement stores.
- `market.trade`: exchange ore and future crafted goods for coin while preserving ownership and value accounting.
- `governance.vote`: cast one-member-one-vote decisions on public proposals.

## Contributor opportunities

1. **Economy design** — propose balanced prices, materials, and trade scenarios for the next release journal.
2. **Governance scenarios** — write proposals, quorum rules, and council playbooks.
3. **Settlement content** — add public workshop, storage, roads, and frontier lore records.
4. **Verification review** — independently re-run package, replay, remote, and attestation checks.
5. **Operator onboarding** — turn the RC1 guide into provider-specific runbooks.

## How to join

1. Find **Frontier Settlement RC1** in the public portal fixture.
2. Review proof status and the public proof bundle.
3. Ask the operator for the current live endpoint from the deployment manifest.
4. Pick a contributor lane above and submit a small deterministic scenario proposal.

## Acceptance bar for new content

New content must describe the player-facing intent, the deterministic mutation used, the expected receipt or governance outcome, and the proof files that should change after replay. If a change cannot be replayed from genesis, it is not ready for Frontier Settlement RC1.
