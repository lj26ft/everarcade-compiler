# Onboarding Report

Status: PASS

First-time developer simulation: Rust knowledge, no EverArcade knowledge.

Questions:

- Can they find templates? PASS via `everarcade world templates`.
- Can they find RustRigs? PASS via `everarcade world rustrigs`.
- Can they build a world? PASS via `everarcade world init --template frontier` and `everarcade world run`.
- Can they package a world? PASS via `everarcade world package` producing `dist/world.evr`.
- Can they verify a world? PASS via `everarcade world verify` producing `WORLD VERIFY: PASS`.

Expected Time To First World: under 15 minutes with dependencies already installed; stretch under 10 minutes for a warm Rust build cache.
