#!/usr/bin/env bash
set -euo pipefail

failed=0

for file in execution-core/tests/*.rs everarcade-host/tests/*.rs; do
  [ -e "$file" ] || continue
  if ! grep -Eq "#\[test\]|#\[tokio::test\]|#\[async_std::test\]" "$file"; then
    echo "ERROR: no test functions found in $file"
    failed=1
  fi
done

exit "$failed"
