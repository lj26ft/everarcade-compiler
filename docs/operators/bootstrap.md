# EverNode Operator Bootstrap Guide

## Purpose
Bootstrap an EverNode identity and prepare it to join the Arena Vanguard deployment federation.

## Bootstrap Steps
1. Choose a stable node name such as `evernode-a` or `evernode-b`.
2. Read `deployment/evernode/runtime/operator-manifest.toml` and confirm the expected workflow documents are present.
3. Unpack `arena-vanguard-runtime.tar.gz` into the node runtime directory.
4. Unpack `arena-vanguard-world.tar.gz` into the world state directory.
5. Record the replay root, world root, checkpoint root, and continuity root from the world manifest.

## Verification
Run `bash scripts/run_operator_validation.sh --offline --locked` and keep the emitted report as the bootstrap receipt.
