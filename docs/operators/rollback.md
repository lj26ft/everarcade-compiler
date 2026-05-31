# EverNode Operator Rollback Guide

## Purpose
Rollback a failed upgrade to the previously verified Arena Vanguard deployment package.

## Rollback Steps
1. Stop the upgraded node.
2. Restore the previous runtime, world, and deployment packages.
3. Restore the last known-good checkpoint.
4. Restart the node.
5. Rejoin federation and verify replay sync and checkpoint sync.

## Rollback Gate
Rollback is complete only when the continuity root matches the last known-good receipt and federation health is restored.
