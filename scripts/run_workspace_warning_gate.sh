#!/usr/bin/env bash
set -euo pipefail

export RUSTFLAGS="-D warnings"

CLEAN_BETWEEN_STAGES=0
CHUNK_SIZE="${WORKSPACE_WARNING_GATE_CHUNK_SIZE:-6}"
TARGET_DIR="${CARGO_TARGET_DIR:-target}"
ARGS=()

while (($#)); do
  case "$1" in
    --clean-between-stages)
      CLEAN_BETWEEN_STAGES=1
      shift
      ;;
    --chunk-size)
      CHUNK_SIZE="$2"
      shift 2
      ;;
    *)
      ARGS+=("$1")
      shift
      ;;
  esac
done

print_disk_usage() {
  local label="$1"
  printf '[workspace-warning-gate] DISK %s\n' "$label"
  df -h .
  if [[ -d "$TARGET_DIR" ]]; then
    du -sh "$TARGET_DIR"
  else
    printf '[workspace-warning-gate] INFO target dir %s does not exist yet\n' "$TARGET_DIR"
  fi
}

clean_target_dir() {
  if [[ "$CLEAN_BETWEEN_STAGES" -eq 1 && -d "$TARGET_DIR" ]]; then
    printf '[workspace-warning-gate] CLEAN removing %s\n' "$TARGET_DIR"
    rm -rf "$TARGET_DIR"
  fi
}

run_stage() {
  local label="$1"
  shift

  print_disk_usage "before ${label}"
  clean_target_dir

  printf '[workspace-warning-gate] START %s\n' "$label"
  "$@"
  printf '[workspace-warning-gate] PASS  %s\n' "$label"

  print_disk_usage "after ${label}"
}

mapfile -t PACKAGES < <(
  cargo metadata --format-version=1 --no-deps \
  | python3 -c 'import json,sys; d=json.load(sys.stdin); r=d["resolve"]; ws=set(r["root"] for _ in [0] if r and r.get("root"));
for p in d["packages"]:
  if p["id"] in d["workspace_members"]:
    print(p["name"])'
)

if [[ "${#PACKAGES[@]}" -eq 0 ]]; then
  echo '[workspace-warning-gate] ERROR no workspace packages found' >&2
  exit 1
fi

run_stage workspace_check cargo check --workspace --all-targets "${ARGS[@]}"

stage_index=1
for ((i=0; i<${#PACKAGES[@]}; i+=CHUNK_SIZE)); do
  stage_packages=("${PACKAGES[@]:i:CHUNK_SIZE}")
  cmd=(cargo test --no-run)
  cmd+=("${ARGS[@]}")
  for pkg in "${stage_packages[@]}"; do
    cmd+=(-p "$pkg")
  done

  run_stage "segmented_test_compile_${stage_index}" "${cmd[@]}"
  ((stage_index++))
done

printf '[workspace-warning-gate] COMPLETE all stages\n'
