#!/usr/bin/env bash
set -euo pipefail

echo '== Directory sizes (depth 1) =='
du -h --max-depth=1 . | sort -h

echo
echo '== Files larger than 10M (depth 5) =='
find . -maxdepth 5 -type f -size +10M -print

echo
echo '== Git status (short) =='
git status --short

echo
echo '== Tracked generated artifact extensions =='
git ls-files | grep -E '\.(bin|wasm|log|tmp|out)$' || true
