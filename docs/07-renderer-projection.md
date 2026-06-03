# 07. Renderer Projection

## Purpose

The renderer projection layer turns authoritative runtime state and replay material into visible client experiences. It is intentionally non-authoritative.

## Status

Renderer and historical projection domains are scaffold-level runtime domains unless a specific capability is listed otherwise in `runtime-capabilities.md`.

## Responsibilities

- consume runtime projections, replay data, or client bridge outputs;
- display game state to players or observers;
- support debugging and replay visualization;
- tolerate missing or delayed projection data without changing authority.

## Non-Responsibilities

The renderer must not define canonical state, repair divergence, certify receipts, or bypass runtime input admission.

## Future Work

Renderer streaming needs stable projection protocols, backpressure handling, replay alignment, browser smoke gates, and explicit separation between visual interpolation and authoritative state.
