# EverNode Operator Recover Guide

## Purpose
Recover the runtime after crash, restart, checkpoint restore, or world restore while preserving deterministic roots.

## Recovery Steps
1. Stop the failed node process.
2. Preserve the last receipt and checkpoint files.
3. Restore from `arena-vanguard-world.tar.gz` when world files are missing.
4. Restore from the checkpoint root recorded in `world-manifest.toml` when state files are corrupt.
5. Restart the node and compare replay root, world root, and continuity root against the deployment receipts.

## Verification
Recovery succeeds only when the restored replay root, world root, and continuity root equal the pre-failure values.
