# EverNode Operator Install Guide

## Purpose
Install the deterministic EverNode runtime prerequisites for the Arena Vanguard launch candidate without relying on simulated services.

## Prerequisites
- Linux host with TCP networking between operators.
- Rust toolchain already provisioned by the release environment.
- Access to the checked-out EverArcade repository and `deployment/evernode/runtime/` artifacts.

## Install Steps
1. Set `CARGO_BUILD_JOBS=1` for reproducible low-resource validation.
2. Run `bash scripts/vendor_deps.sh` before offline validation.
3. Verify the packages with `bash scripts/run_evernode_deployment_gate.sh --offline --locked`.
4. Confirm `arena-vanguard-runtime.tar.gz`, `arena-vanguard-world.tar.gz`, and `arena-vanguard-deployment.tar.gz` are present with matching `.sha256`, `.sig`, and receipt files.

## Operator Exit Criteria
Installation is complete only when package, hash, signature, and receipt verification pass locally.
