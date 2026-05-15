#!/usr/bin/env bash
set -euo pipefail
cargo test -q --test networked_receipt_propagation_tests
