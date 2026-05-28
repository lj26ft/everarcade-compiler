#!/usr/bin/env bash
set -euo pipefail
test -f runtime/deployment/evernode.rs
test -f runtime/config/deployment.toml
echo "validation=ok script=$(basename "$0") package=everarcade-sovereign-runtime append_only=true"
