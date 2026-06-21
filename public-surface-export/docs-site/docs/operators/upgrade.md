# EverNode Operator Upgrade Guide

## Purpose
Upgrade an EverNode deployment without breaking replay or checkpoint continuity.

## Upgrade Steps
1. Verify the replacement runtime package and deployment manifest.
2. Drain new sessions from the node.
3. Capture the current replay root, checkpoint root, and continuity root.
4. Apply the replacement package.
5. Restart and verify roots before rejoining federation.

## Roll-Forward Gate
Upgrade is permitted only if package verification, replay continuity, checkpoint continuity, and federation sync pass.
