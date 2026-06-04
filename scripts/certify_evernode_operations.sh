#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"; cd "$ROOT"
REPORT_DIR="$ROOT/reports"; REPORT="$REPORT_DIR/evernode_operations_certification_report.txt"; mkdir -p "$REPORT_DIR"
run_step() { local label="$1" script="$2" var="$3" output; if output="$(bash "$script" 2>&1)"; then printf -v "$var" PASS; else printf -v "$var" FAIL; fi; printf -v "${var}_output" '%s' "$output"; }
package_build="FAIL"; install="FAIL"; start="FAIL"; health="FAIL"; recovery="FAIL"; upgrade="FAIL"; stop="FAIL"; overall="FAIL"
run_step "Package Build" scripts/build_evernode_package.sh package_build
run_step "Install" scripts/install_evernode_package.sh install
run_step "Start" scripts/evernode_start.sh start
run_step "Health" scripts/evernode_health.sh health
run_step "Recovery" scripts/evernode_recover.sh recovery
run_step "Upgrade" scripts/evernode_upgrade.sh upgrade
run_step "Stop" scripts/evernode_stop.sh stop
if [[ "$package_build" == PASS && "$install" == PASS && "$start" == PASS && "$health" == PASS && "$recovery" == PASS && "$upgrade" == PASS && "$stop" == PASS ]]; then overall="PASS"; fi
cat > "$REPORT" <<REPORT_BODY
Evernode Operations Certification Report
Package Build: $package_build
Installation: $install
Start: $start
Health: $health
Recovery: $recovery
Upgrade: $upgrade
Stop: $stop
Evernode Operations: $overall

Package Build Output:
$package_build_output

Install Output:
$install_output

Start Output:
$start_output

Health Output:
$health_output

Recovery Output:
$recovery_output

Upgrade Output:
$upgrade_output

Stop Output:
$stop_output
REPORT_BODY
echo "Evernode Operations: $overall"
[[ "$overall" == PASS ]]
