# 06. Federation Runtime

## Purpose

Federation coordinates multiple runtime nodes without abandoning deterministic authority. It exchanges verifiable artifacts: roots, receipts, checkpoints, replay windows, peer status, and recovery material.

## Current Status

Federation is partial. The repository contains modules, tests, fixtures, reports, and scaffolding for peer discovery, synchronization, convergence, recovery, partition execution, and receipt propagation. Production multi-host authority is not certified.

## Federation Responsibilities

- discover peers and capabilities;
- exchange receipts and checkpoints;
- detect gaps and divergence;
- compare roots;
- support recovery and rejoin flows;
- preserve lineage across peers.

## Non-Responsibilities

Federation must not invent state, override runtime receipts, or trust renderer output. A federation peer can propose or supply artifacts; runtime verification decides whether they are accepted.

## Production Gates

Multi-host federation requires authenticated peer policy, adversarial partition tests, recovery drills, operator runbooks, observability, rate limits, deployment automation, and release certification.
