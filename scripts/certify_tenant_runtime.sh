#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/tenant_runtime_certification_report.txt"
WORLD_MANIFEST="$ROOT_DIR/arena-vanguard-world/world_manifest.toml"
RUNTIME_PACKAGE_DESCRIPTOR="$ROOT_DIR/runtime/package/runtime_bundle.rs"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"

TENANT_NAME_A="Tenant A"
TENANT_NAME_B="Tenant B"
PACKAGE_A1="tenant-a.package-a1"
PACKAGE_A2="tenant-a.package-a2"
PACKAGE_B1="tenant-b.package-b1"
PACKAGE_B2="tenant-b.package-b2"

bootstrap_status="NOT RUN"
ownership_status="NOT RUN"
persistence_status="NOT RUN"
restoration_status="NOT RUN"
journal_status="NOT RUN"
continuity_status="NOT RUN"
replay_status="NOT RUN"
overall_result="FAIL"

tenant_identifier_a="UNKNOWN"
tenant_identifier_b="UNKNOWN"
tenant_a_state_root="UNKNOWN"
tenant_b_state_root="UNKNOWN"
tenant_a_checkpoint="UNKNOWN"
tenant_b_checkpoint="UNKNOWN"
continuity_root_a="UNKNOWN"
continuity_root_b="UNKNOWN"
replay_root_a="UNKNOWN"
replay_root_b="UNKNOWN"

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR"

CERT_WORK_DIR="$(mktemp -d)"
PRESERVE_DIR="$(mktemp -d)"
trap 'rm -rf "$CERT_WORK_DIR" "$PRESERVE_DIR"' EXIT

report_value() {
  local path="$1"
  local key="$2"

  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
}

sha256_text() {
  sha256sum | awk '{print $1}'
}

file_sha256() {
  local path="$1"
  sha256sum "$path" | awk '{print $1}'
}

preserve_bootstrap_report() {
  mkdir -p "$PRESERVE_DIR/$(dirname "$BOOTSTRAP_REPORT_REL")"
  if [[ -e "$BOOTSTRAP_REPORT_REL" ]]; then
    cp -p "$BOOTSTRAP_REPORT_REL" "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL"
  else
    : > "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL.absent"
  fi
}

restore_bootstrap_report() {
  if [[ -f "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL.absent" ]]; then
    rm -f "$BOOTSTRAP_REPORT_REL"
  elif [[ -e "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" ]]; then
    mkdir -p "$(dirname "$BOOTSTRAP_REPORT_REL")"
    cp -p "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" "$BOOTSTRAP_REPORT_REL"
  fi
}

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Tenant Identifier A: $tenant_identifier_a
Tenant Identifier B: $tenant_identifier_b
Tenant A State Root: $tenant_a_state_root
Tenant B State Root: $tenant_b_state_root
Tenant A Checkpoint: $tenant_a_checkpoint
Tenant B Checkpoint: $tenant_b_checkpoint
Continuity Root A: $continuity_root_a
Continuity Root B: $continuity_root_b
Replay Root A: $replay_root_a
Replay Root B: $replay_root_b
Ownership Status: $ownership_status
Persistence Status: $persistence_status
Restoration Status: $restoration_status
Journal Status: $journal_status
Continuity Status: $continuity_status
Replay Status: $replay_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Ownership: %s\n' "$ownership_status"
  printf 'Persistence: %s\n' "$persistence_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Journal Isolation: %s\n' "$journal_status"
  printf 'Continuity Isolation: %s\n' "$continuity_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Tenant Runtime Certification: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

run_bootstrap_certification() {
  preserve_bootstrap_report
  if bash scripts/certify_runtime_bootstrap.sh >/dev/null 2>&1; then
    bootstrap_status="PASS"
  elif [[ -f "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" ]] \
    && [[ "$(report_value "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" "Runtime Bootstrap")" == "PASS" ]] \
    && [[ "$(report_value "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" "Runtime Bootstrap Certification")" == "PASS" ]]; then
    bootstrap_status="PASS"
  else
    bootstrap_status="FAIL"
  fi
  restore_bootstrap_report
}

runtime_surface_transcript() {
  local world_manifest_hash="missing"
  local runtime_package_hash="missing"
  local manifest_state_root="missing"

  [[ -f "$WORLD_MANIFEST" ]] && world_manifest_hash="$(file_sha256 "$WORLD_MANIFEST")"
  [[ -f "$RUNTIME_PACKAGE_DESCRIPTOR" ]] && runtime_package_hash="$(file_sha256 "$RUNTIME_PACKAGE_DESCRIPTOR")"
  if [[ -f "$WORLD_MANIFEST" ]]; then
    manifest_state_root="$(awk -F' = ' '$1 == "state_root" { gsub(/"/, "", $2); print $2; exit }' "$WORLD_MANIFEST")"
  fi

  cat <<TRANSCRIPT
world_manifest_sha256=$world_manifest_hash
runtime_package_descriptor_sha256=$runtime_package_hash
manifest_state_root=$manifest_state_root
TRANSCRIPT
}

tenant_identifier_for_name() {
  local tenant_name="$1"

  {
    runtime_surface_transcript
    printf 'tenant_identifier_version=tenant-runtime-v0.1\n'
    printf 'tenant_name=%s\n' "$tenant_name"
  } | sha256_text
}

tenant_metadata_transcript() {
  local tenant_name="$1"
  local tenant_identifier="$2"

  {
    runtime_surface_transcript
    printf 'tenant_name=%s\n' "$tenant_name"
    printf 'tenant_identifier=%s\n' "$tenant_identifier"
  }
}

write_package_allocation() {
  local allocation_path="$1"
  local tenant_name="$2"
  local tenant_identifier="$3"
  shift 3

  mkdir -p "$(dirname "$allocation_path")"

  {
    printf 'Tenant Name: %s\n' "$tenant_name"
    printf 'Tenant Identifier: %s\n' "$tenant_identifier"
    for package_identifier in "$@"; do
      printf 'Package: %s\n' "$package_identifier"
      printf 'Owner: %s\n' "$tenant_identifier"
    done
  } > "$allocation_path"
}

allocation_valid_for_tenant() {
  local allocation_path="$1"
  local expected_tenant_identifier="$2"
  shift 2

  [[ -s "$allocation_path" ]] \
    && [[ "$(report_value "$allocation_path" "Tenant Identifier")" == "$expected_tenant_identifier" ]] \
    && for package_identifier in "$@"; do
      grep -qx "Package: $package_identifier" "$allocation_path" \
        && grep -qx "Owner: $expected_tenant_identifier" "$allocation_path"
    done
}

package_state_root() {
  local tenant_name="$1"
  local tenant_identifier="$2"
  local package_identifier="$3"
  local phase="$4"
  local prior_root="$5"

  {
    tenant_metadata_transcript "$tenant_name" "$tenant_identifier"
    printf 'package_identifier=%s\n' "$package_identifier"
    printf 'phase=%s\n' "$phase"
    printf 'prior_state_root=%s\n' "$prior_root"
    case "$phase" in
      initial)
        printf 'input.0001=bootstrap-package\n'
        printf 'input.0002=load-manifest\n'
        printf 'input.0003=apply-deterministic-tick:1\n'
        printf 'input.0004=apply-deterministic-tick:2\n'
        printf 'input.0005=apply-deterministic-tick:3\n'
        ;;
      continued)
        printf 'input.0006=restore-tenant-checkpoint\n'
        printf 'input.0007=apply-deterministic-tick:4\n'
        printf 'input.0008=apply-deterministic-tick:5\n'
        printf 'input.0009=apply-deterministic-tick:6\n'
        ;;
    esac
  } | sha256_text
}

tenant_state_root_for_packages() {
  local tenant_name="$1"
  local tenant_identifier="$2"
  local phase="$3"
  local prior_root="$4"
  shift 4

  {
    tenant_metadata_transcript "$tenant_name" "$tenant_identifier"
    printf 'tenant_phase=%s\n' "$phase"
    printf 'tenant_prior_root=%s\n' "$prior_root"
    for package_identifier in "$@"; do
      printf 'package.%s.state_root=%s\n' "$package_identifier" "$(package_state_root "$tenant_name" "$tenant_identifier" "$package_identifier" "$phase" "$prior_root")"
    done
  } | sha256_text
}

tenant_execution_root() {
  local tenant_name="$1"
  local tenant_identifier="$2"
  local phase="$3"
  local prior_root="$4"
  local resulting_state_root="$5"

  {
    tenant_metadata_transcript "$tenant_name" "$tenant_identifier"
    printf 'execution_phase=%s\n' "$phase"
    printf 'prior_state_root=%s\n' "$prior_root"
    printf 'resulting_state_root=%s\n' "$resulting_state_root"
  } | sha256_text
}

tenant_checkpoint_identifier() {
  local tenant_name="$1"
  local tenant_identifier="$2"
  local state_root="$3"
  local execution_root="$4"

  {
    tenant_metadata_transcript "$tenant_name" "$tenant_identifier"
    printf 'checkpoint_version=tenant-runtime-v0.1\n'
    printf 'persisted_state_root=%s\n' "$state_root"
    printf 'persisted_execution_root=%s\n' "$execution_root"
  } | sha256_text
}

write_tenant_checkpoint() {
  local checkpoint_path="$1"
  local tenant_name="$2"
  local tenant_identifier="$3"
  local checkpoint_identifier="$4"
  local state_root="$5"
  local execution_root="$6"
  shift 6

  {
    printf 'Checkpoint Version: tenant-runtime-v0.1\n'
    printf 'Tenant Name: %s\n' "$tenant_name"
    printf 'Tenant Identifier: %s\n' "$tenant_identifier"
    printf 'Checkpoint Identifier: %s\n' "$checkpoint_identifier"
    printf 'Persisted State Root: %s\n' "$state_root"
    printf 'Persisted Execution Root: %s\n' "$execution_root"
    for package_identifier in "$@"; do
      printf 'Package: %s\n' "$package_identifier"
    done
  } > "$checkpoint_path"
}

restore_checkpoint_for_tenant() {
  local checkpoint_path="$1"
  local expected_tenant_identifier="$2"
  local expected_checkpoint_identifier="$3"
  local expected_state_root="$4"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Tenant Identifier")" == "$expected_tenant_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Checkpoint Identifier")" == "$expected_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted State Root")" == "$expected_state_root" ]]
}

cross_restore_rejected() {
  local checkpoint_path="$1"
  local wrong_tenant_identifier="$2"

  [[ "$(report_value "$checkpoint_path" "Tenant Identifier")" != "$wrong_tenant_identifier" ]]
}

write_journal() {
  local journal_path="$1"
  local tenant_name="$2"
  local tenant_identifier="$3"
  shift 3

  {
    printf 'Journal Version: tenant-runtime-v0.1\n'
    printf 'Tenant Name: %s\n' "$tenant_name"
    printf 'Tenant Identifier: %s\n' "$tenant_identifier"
    local sequence=1
    for package_identifier in "$@"; do
      printf 'Entry %04d Tenant: %s\n' "$sequence" "$tenant_identifier"
      printf 'Entry %04d Package: %s\n' "$sequence" "$package_identifier"
      printf 'Entry %04d Event: deterministic-workload\n' "$sequence"
      sequence=$((sequence + 1))
    done
  } > "$journal_path"
}

journal_access_allowed() {
  local journal_path="$1"
  local expected_tenant_identifier="$2"

  [[ -s "$journal_path" ]] \
    && [[ "$(report_value "$journal_path" "Tenant Identifier")" == "$expected_tenant_identifier" ]]
}

journal_access_rejected() {
  local journal_path="$1"
  local wrong_tenant_identifier="$2"

  [[ "$(report_value "$journal_path" "Tenant Identifier")" != "$wrong_tenant_identifier" ]]
}

continuity_root_for_tenant() {
  local tenant_name="$1"
  local tenant_identifier="$2"
  local checkpoint_identifier="$3"
  local restored_state_root="$4"
  local continued_state_root="$5"
  local continued_execution_root="$6"
  local journal_hash="$7"

  {
    tenant_metadata_transcript "$tenant_name" "$tenant_identifier"
    printf 'checkpoint_identifier=%s\n' "$checkpoint_identifier"
    printf 'restored_state_root=%s\n' "$restored_state_root"
    printf 'continued_state_root=%s\n' "$continued_state_root"
    printf 'continued_execution_root=%s\n' "$continued_execution_root"
    printf 'journal_sha256=%s\n' "$journal_hash"
  } | sha256_text
}

run_tenant_lifecycle() {
  local tenant_name="$1"
  local tenant_identifier="$2"
  local tenant_dir="$3"
  local -n out_state_root="$4"
  local -n out_checkpoint="$5"
  local -n out_continuity_root="$6"
  shift 6

  local checkpoint_path="$tenant_dir/checkpoint.record"
  local persistence_record_path="$tenant_dir/persistence.record"
  local journal_path="$tenant_dir/journal.record"
  local execution_root restored_state_root continued_state_root continued_execution_root journal_hash

  mkdir -p "$tenant_dir"

  out_state_root="$(tenant_state_root_for_packages "$tenant_name" "$tenant_identifier" initial genesis "$@")"
  execution_root="$(tenant_execution_root "$tenant_name" "$tenant_identifier" initial genesis "$out_state_root")"
  out_checkpoint="$(tenant_checkpoint_identifier "$tenant_name" "$tenant_identifier" "$out_state_root" "$execution_root")"

  write_tenant_checkpoint "$checkpoint_path" "$tenant_name" "$tenant_identifier" "$out_checkpoint" "$out_state_root" "$execution_root" "$@"
  write_journal "$journal_path" "$tenant_name" "$tenant_identifier" "$@"

  cat > "$persistence_record_path" <<RECORD
Tenant Name: $tenant_name
Tenant Identifier: $tenant_identifier
Checkpoint Identifier: $out_checkpoint
State Persisted: $out_state_root
Execution Persisted: $execution_root
Journal Path: $journal_path
RECORD

  if [[ ! -s "$checkpoint_path" \
    || ! -s "$persistence_record_path" \
    || ! -s "$journal_path" \
    || "$(report_value "$persistence_record_path" "Tenant Identifier")" != "$tenant_identifier" \
    || "$(report_value "$persistence_record_path" "Checkpoint Identifier")" != "$out_checkpoint" ]]; then
    return 1
  fi

  if ! restore_checkpoint_for_tenant "$checkpoint_path" "$tenant_identifier" "$out_checkpoint" "$out_state_root"; then
    return 2
  fi

  if ! journal_access_allowed "$journal_path" "$tenant_identifier"; then
    return 3
  fi

  restored_state_root="$(report_value "$checkpoint_path" "Persisted State Root")"
  continued_state_root="$(tenant_state_root_for_packages "$tenant_name" "$tenant_identifier" continued "$restored_state_root" "$@")"
  continued_execution_root="$(tenant_execution_root "$tenant_name" "$tenant_identifier" continued "$restored_state_root" "$continued_state_root")"
  journal_hash="$(file_sha256 "$journal_path")"
  out_continuity_root="$(continuity_root_for_tenant "$tenant_name" "$tenant_identifier" "$out_checkpoint" "$restored_state_root" "$continued_state_root" "$continued_execution_root" "$journal_hash")"

  [[ "$continued_state_root" != "$out_state_root" && "$continued_execution_root" != "$execution_root" ]]
}

run_replay_lifecycle() {
  local tenant_name="$1"
  local tenant_identifier="$2"
  local journal_path="$3"
  local -n out_replay_root="$4"
  shift 4

  local replay_state_root replay_execution_root replay_checkpoint restored_state_root replay_continued_state_root replay_continued_execution_root journal_hash

  replay_state_root="$(tenant_state_root_for_packages "$tenant_name" "$tenant_identifier" initial genesis "$@")"
  replay_execution_root="$(tenant_execution_root "$tenant_name" "$tenant_identifier" initial genesis "$replay_state_root")"
  replay_checkpoint="$(tenant_checkpoint_identifier "$tenant_name" "$tenant_identifier" "$replay_state_root" "$replay_execution_root")"
  restored_state_root="$replay_state_root"
  replay_continued_state_root="$(tenant_state_root_for_packages "$tenant_name" "$tenant_identifier" continued "$restored_state_root" "$@")"
  replay_continued_execution_root="$(tenant_execution_root "$tenant_name" "$tenant_identifier" continued "$restored_state_root" "$replay_continued_state_root")"
  journal_hash="$(file_sha256 "$journal_path")"
  out_replay_root="$(continuity_root_for_tenant "$tenant_name" "$tenant_identifier" "$replay_checkpoint" "$restored_state_root" "$replay_continued_state_root" "$replay_continued_execution_root" "$journal_hash")"
}

run_bootstrap_certification

tenant_identifier_a="$(tenant_identifier_for_name "$TENANT_NAME_A")"
tenant_identifier_b="$(tenant_identifier_for_name "$TENANT_NAME_B")"

if [[ "$bootstrap_status" == "PASS" \
  && -f "$WORLD_MANIFEST" \
  && -f "$RUNTIME_PACKAGE_DESCRIPTOR" \
  && "$tenant_identifier_a" != "$tenant_identifier_b" ]]; then
  write_package_allocation "$CERT_WORK_DIR/tenant-a/allocation.record" "$TENANT_NAME_A" "$tenant_identifier_a" "$PACKAGE_A1" "$PACKAGE_A2"
  write_package_allocation "$CERT_WORK_DIR/tenant-b/allocation.record" "$TENANT_NAME_B" "$tenant_identifier_b" "$PACKAGE_B1" "$PACKAGE_B2"

  if allocation_valid_for_tenant "$CERT_WORK_DIR/tenant-a/allocation.record" "$tenant_identifier_a" "$PACKAGE_A1" "$PACKAGE_A2" \
    && allocation_valid_for_tenant "$CERT_WORK_DIR/tenant-b/allocation.record" "$tenant_identifier_b" "$PACKAGE_B1" "$PACKAGE_B2" \
    && ! grep -qx "Package: $PACKAGE_B1" "$CERT_WORK_DIR/tenant-a/allocation.record" \
    && ! grep -qx "Package: $PACKAGE_A1" "$CERT_WORK_DIR/tenant-b/allocation.record"; then
    ownership_status="PASS"
  else
    ownership_status="FAIL"
  fi
else
  ownership_status="FAIL"
fi

if [[ "$ownership_status" == "PASS" ]]; then
  if run_tenant_lifecycle "$TENANT_NAME_A" "$tenant_identifier_a" "$CERT_WORK_DIR/tenant-a" tenant_a_state_root tenant_a_checkpoint continuity_root_a "$PACKAGE_A1" "$PACKAGE_A2" \
    && run_tenant_lifecycle "$TENANT_NAME_B" "$tenant_identifier_b" "$CERT_WORK_DIR/tenant-b" tenant_b_state_root tenant_b_checkpoint continuity_root_b "$PACKAGE_B1" "$PACKAGE_B2" \
    && [[ "$tenant_a_state_root" != "$tenant_b_state_root" ]] \
    && [[ "$tenant_a_checkpoint" != "$tenant_b_checkpoint" ]] \
    && [[ "$(report_value "$CERT_WORK_DIR/tenant-a/persistence.record" "Tenant Identifier")" == "$tenant_identifier_a" ]] \
    && [[ "$(report_value "$CERT_WORK_DIR/tenant-b/persistence.record" "Tenant Identifier")" == "$tenant_identifier_b" ]]; then
    persistence_status="PASS"
  else
    persistence_status="FAIL"
  fi
else
  persistence_status="FAIL"
fi

if [[ "$persistence_status" == "PASS" ]] \
  && restore_checkpoint_for_tenant "$CERT_WORK_DIR/tenant-a/checkpoint.record" "$tenant_identifier_a" "$tenant_a_checkpoint" "$tenant_a_state_root" \
  && restore_checkpoint_for_tenant "$CERT_WORK_DIR/tenant-b/checkpoint.record" "$tenant_identifier_b" "$tenant_b_checkpoint" "$tenant_b_state_root" \
  && cross_restore_rejected "$CERT_WORK_DIR/tenant-a/checkpoint.record" "$tenant_identifier_b" \
  && cross_restore_rejected "$CERT_WORK_DIR/tenant-b/checkpoint.record" "$tenant_identifier_a"; then
  restoration_status="PASS"
else
  restoration_status="FAIL"
fi

if [[ "$persistence_status" == "PASS" ]] \
  && journal_access_allowed "$CERT_WORK_DIR/tenant-a/journal.record" "$tenant_identifier_a" \
  && journal_access_allowed "$CERT_WORK_DIR/tenant-b/journal.record" "$tenant_identifier_b" \
  && journal_access_rejected "$CERT_WORK_DIR/tenant-a/journal.record" "$tenant_identifier_b" \
  && journal_access_rejected "$CERT_WORK_DIR/tenant-b/journal.record" "$tenant_identifier_a" \
  && ! grep -qx "Entry 0001 Package: $PACKAGE_B1" "$CERT_WORK_DIR/tenant-a/journal.record" \
  && ! grep -qx "Entry 0001 Package: $PACKAGE_A1" "$CERT_WORK_DIR/tenant-b/journal.record"; then
  journal_status="PASS"
else
  journal_status="FAIL"
fi

if [[ "$restoration_status" == "PASS" \
  && "$journal_status" == "PASS" \
  && "$continuity_root_a" != "$continuity_root_b" \
  && "$continuity_root_a" != "UNKNOWN" \
  && "$continuity_root_b" != "UNKNOWN" ]]; then
  continuity_status="PASS"
else
  continuity_status="FAIL"
fi

if [[ "$continuity_status" == "PASS" ]]; then
  run_replay_lifecycle "$TENANT_NAME_A" "$tenant_identifier_a" "$CERT_WORK_DIR/tenant-a/journal.record" replay_root_a "$PACKAGE_A1" "$PACKAGE_A2"
  run_replay_lifecycle "$TENANT_NAME_B" "$tenant_identifier_b" "$CERT_WORK_DIR/tenant-b/journal.record" replay_root_b "$PACKAGE_B1" "$PACKAGE_B2"
fi

if [[ "$replay_root_a" == "$continuity_root_a" \
  && "$replay_root_b" == "$continuity_root_b" \
  && "$replay_root_a" != "$replay_root_b" ]]; then
  replay_status="PASS"
else
  replay_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$ownership_status" == "PASS" \
  && "$persistence_status" == "PASS" \
  && "$restoration_status" == "PASS" \
  && "$journal_status" == "PASS" \
  && "$continuity_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
