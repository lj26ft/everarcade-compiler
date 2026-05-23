#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core --test persistence_archive_tests --test economic_ledger_tests --test vault_continuity_tests --test replay_compression_tests --test storage_lineage_tests --test inventory_tests --test entity_evolution_tests --test sync_window_tests --test xrpl_anchor_tests --test persistence_restoration_tests --test sovereign_world_persistence_tests -- --nocapture
