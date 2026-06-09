# Network Transport Proof Model

This directory marks the isolated local transport proof surface for the Network Transport & Session Synchronization milestone. The runnable implementation is compiled with the EverArcade runtime crate and exposes the `network-local-session` operator command.

The proof is intentionally local. It models deterministic transport messages and two independent client identities synchronizing through a runtime authority; it does not implement federation, Evernode deployment, WAN networking, multi-lease coordination, or production transport reliability.
