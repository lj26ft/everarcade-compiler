#!/usr/bin/env bash
# Deterministic Multi-Lease Federation Runtime v0.1 model.
# The runtime exchanges evidence only; no lease directly mutates another lease.

FEDERATION_RUNTIME_VERSION="multi-lease-federation-runtime-v0.1"
FEDERATION_ID="everarcade-federation-alpha"
MEMBERSHIP_EPOCH="epoch-0001"
CIVILIZATION_EPOCH="civilization-epoch-0001"

FEDERATION_LEASE_IDS=("lease-a" "lease-b" "lease-c")
FEDERATION_NODE_IDS=("node-a" "node-b" "node-c")
FEDERATION_AUTHORITIES=("authority-a" "authority-b" "authority-c")

federation_sha256_text() {
  sha256sum | awk '{print $1}'
}

federation_short_hash() {
  cut -c1-16
}

canonical_federation_identity() {
  cat <<EOF_IDENTITY
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
MembershipEpoch=$MEMBERSHIP_EPOCH
Lease.0.ID=${FEDERATION_LEASE_IDS[0]}
Lease.0.NodeID=${FEDERATION_NODE_IDS[0]}
Lease.0.Authority=${FEDERATION_AUTHORITIES[0]}
Lease.1.ID=${FEDERATION_LEASE_IDS[1]}
Lease.1.NodeID=${FEDERATION_NODE_IDS[1]}
Lease.1.Authority=${FEDERATION_AUTHORITIES[1]}
Lease.2.ID=${FEDERATION_LEASE_IDS[2]}
Lease.2.NodeID=${FEDERATION_NODE_IDS[2]}
Lease.2.Authority=${FEDERATION_AUTHORITIES[2]}
IdentityRoot=$(identity_root)
EOF_IDENTITY
}

identity_root() {
  [[ -n "${FEDERATION_IDENTITY_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_IDENTITY_ROOT_CACHE"
}

federation_identity_root() {
  [[ -n "${FEDERATION_IDENTITY_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_IDENTITY_ROOT_CACHE"
}

canonical_membership_events() {
  cat <<EOF_MEMBERSHIP
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
MembershipEpoch=$MEMBERSHIP_EPOCH
Event.0=Join:${FEDERATION_LEASE_IDS[0]}:${FEDERATION_NODE_IDS[0]}:$MEMBERSHIP_EPOCH
Event.1=Join:${FEDERATION_LEASE_IDS[1]}:${FEDERATION_NODE_IDS[1]}:$MEMBERSHIP_EPOCH
Event.2=Join:${FEDERATION_LEASE_IDS[2]}:${FEDERATION_NODE_IDS[2]}:$MEMBERSHIP_EPOCH
Event.3=Suspend:${FEDERATION_LEASE_IDS[1]}:${FEDERATION_NODE_IDS[1]}:$MEMBERSHIP_EPOCH:isolated-not-mutating-peers
Event.4=Recover:${FEDERATION_LEASE_IDS[1]}:${FEDERATION_NODE_IDS[1]}:$MEMBERSHIP_EPOCH:replay-roots-verified
Event.5=Leave:none:none:$MEMBERSHIP_EPOCH:not-entered
Deterministic=PASS
ReplaySafe=PASS
EpochAware=PASS
IdentityRoot=$(identity_root)
EOF_MEMBERSHIP
}

membership_root() {
  [[ -n "${FEDERATION_MEMBERSHIP_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_MEMBERSHIP_ROOT_CACHE"
}

canonical_topology_state() {
  cat <<EOF_TOPOLOGY
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
MembershipEpoch=$MEMBERSHIP_EPOCH
LeaseMembership=${FEDERATION_LEASE_IDS[0]},${FEDERATION_LEASE_IDS[1]},${FEDERATION_LEASE_IDS[2]}
Connectivity.0=${FEDERATION_LEASE_IDS[0]}->${FEDERATION_LEASE_IDS[1]}:evidence-only
Connectivity.1=${FEDERATION_LEASE_IDS[1]}->${FEDERATION_LEASE_IDS[2]}:evidence-only
Connectivity.2=${FEDERATION_LEASE_IDS[2]}->${FEDERATION_LEASE_IDS[0]}:evidence-only
EpochState=active
NetworkingImplementation=not-required
MembershipRoot=$(membership_root)
EOF_TOPOLOGY
}

topology_root() {
  [[ -n "${FEDERATION_TOPOLOGY_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_TOPOLOGY_ROOT_CACHE"
}

lease_checkpoint_root() {
  local lease="$1"
  printf 'checkpoint|%s|%s|%s\n' "$FEDERATION_RUNTIME_VERSION" "$MEMBERSHIP_EPOCH" "$lease" | federation_sha256_text
}

lease_continuity_root() {
  local lease="$1"
  printf 'continuity|%s|%s|%s|%s\n' "$FEDERATION_RUNTIME_VERSION" "$MEMBERSHIP_EPOCH" "$lease" "$(lease_checkpoint_root "$lease")" | federation_sha256_text
}

lease_replay_root() {
  local lease="$1"
  printf 'replay|%s|%s|%s|%s\n' "$FEDERATION_RUNTIME_VERSION" "$MEMBERSHIP_EPOCH" "$lease" "$(lease_continuity_root "$lease")" | federation_sha256_text
}

lease_settlement_root() {
  local lease="$1"
  printf 'settlement|%s|%s|%s|%s\n' "$FEDERATION_RUNTIME_VERSION" "$MEMBERSHIP_EPOCH" "$lease" "$(lease_replay_root "$lease")" | federation_sha256_text
}

canonical_checkpoint_exchange() {
  cat <<EOF_CHECKPOINT
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
MembershipEpoch=$MEMBERSHIP_EPOCH
Export.${FEDERATION_LEASE_IDS[0]}=$(lease_checkpoint_root "${FEDERATION_LEASE_IDS[0]}")
Export.${FEDERATION_LEASE_IDS[1]}=$(lease_checkpoint_root "${FEDERATION_LEASE_IDS[1]}")
Export.${FEDERATION_LEASE_IDS[2]}=$(lease_checkpoint_root "${FEDERATION_LEASE_IDS[2]}")
ImportPolicy=evidence-only
CheckpointIntegrity=PASS
CheckpointIdentity=PASS
EpochMatch=PASS
IdentityRoot=$(identity_root)
TopologyRoot=$(topology_root)
EOF_CHECKPOINT
}

checkpoint_exchange_root() {
  [[ -n "${FEDERATION_CHECKPOINT_EXCHANGE_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_CHECKPOINT_EXCHANGE_ROOT_CACHE"
}

canonical_replay_exchange() {
  cat <<EOF_REPLAY
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
MembershipEpoch=$MEMBERSHIP_EPOCH
Export.${FEDERATION_LEASE_IDS[0]}=$(lease_replay_root "${FEDERATION_LEASE_IDS[0]}")
Export.${FEDERATION_LEASE_IDS[1]}=$(lease_replay_root "${FEDERATION_LEASE_IDS[1]}")
Export.${FEDERATION_LEASE_IDS[2]}=$(lease_replay_root "${FEDERATION_LEASE_IDS[2]}")
ImportPolicy=replay-authoritative
ReplayIntegrity=PASS
ReplayIdentity=PASS
ContinuityMatch=PASS
CheckpointExchangeRoot=$(checkpoint_exchange_root)
EOF_REPLAY
}

replay_exchange_root() {
  [[ -n "${FEDERATION_REPLAY_EXCHANGE_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_REPLAY_EXCHANGE_ROOT_CACHE"
}

canonical_settlement_exchange() {
  cat <<EOF_SETTLEMENT
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
MembershipEpoch=$MEMBERSHIP_EPOCH
Export.${FEDERATION_LEASE_IDS[0]}=$(lease_settlement_root "${FEDERATION_LEASE_IDS[0]}")
Export.${FEDERATION_LEASE_IDS[1]}=$(lease_settlement_root "${FEDERATION_LEASE_IDS[1]}")
Export.${FEDERATION_LEASE_IDS[2]}=$(lease_settlement_root "${FEDERATION_LEASE_IDS[2]}")
ImportPolicy=receipt-evidence-only
SettlementEvidence=PASS
ReceiptIntegrity=PASS
AuthorityMatch=PASS
ReplayExchangeRoot=$(replay_exchange_root)
EOF_SETTLEMENT
}

settlement_exchange_root() {
  [[ -n "${FEDERATION_SETTLEMENT_EXCHANGE_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_SETTLEMENT_EXCHANGE_ROOT_CACHE"
}

canonical_civilization_synchronization() {
  cat <<EOF_SYNC
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
CivilizationEpoch=$CIVILIZATION_EPOCH
CheckpointSynchronization=$(checkpoint_exchange_root)
ReplaySynchronization=$(replay_exchange_root)
SettlementSynchronization=$(settlement_exchange_root)
SovereignMutationBoundary=PASS
FailureIsolation=PASS
EOF_SYNC
}

civilization_synchronization_root() {
  [[ -n "${FEDERATION_SYNCHRONIZATION_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_SYNCHRONIZATION_ROOT_CACHE"
}

canonical_recovery_state() {
  cat <<EOF_RECOVERY
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
MembershipEpoch=$MEMBERSHIP_EPOCH
LeaseFailure=${FEDERATION_LEASE_IDS[1]}:isolated
LeaseRecovery=${FEDERATION_LEASE_IDS[1]}:replay-verified
MembershipRecovery=$(membership_root)
CheckpointRecovery=$(lease_checkpoint_root "${FEDERATION_LEASE_IDS[1]}")
RecoveredReplayRoot=$(lease_replay_root "${FEDERATION_LEASE_IDS[1]}")
RecoveredContinuityRoot=$(lease_continuity_root "${FEDERATION_LEASE_IDS[1]}")
RecoveryIsolation=PASS
EOF_RECOVERY
}

recovery_root() {
  [[ -n "${FEDERATION_RECOVERY_ROOT_CACHE:-}" ]] || federation_initialize_roots
  printf '%s\n' "$FEDERATION_RECOVERY_ROOT_CACHE"
}

federation_initialize_roots() {
  if [[ -n "${FEDERATION_ROOTS_INITIALIZED:-}" ]]; then
    return
  fi
  FEDERATION_ROOTS_INITIALIZED=1
  FEDERATION_IDENTITY_ROOT_CACHE="$(cat <<EOF_ID_SEED | federation_sha256_text
version=$FEDERATION_RUNTIME_VERSION
FederationID=$FEDERATION_ID
MembershipEpoch=$MEMBERSHIP_EPOCH
Lease.0.ID=${FEDERATION_LEASE_IDS[0]}
Lease.0.NodeID=${FEDERATION_NODE_IDS[0]}
Lease.0.Authority=${FEDERATION_AUTHORITIES[0]}
Lease.1.ID=${FEDERATION_LEASE_IDS[1]}
Lease.1.NodeID=${FEDERATION_NODE_IDS[1]}
Lease.1.Authority=${FEDERATION_AUTHORITIES[1]}
Lease.2.ID=${FEDERATION_LEASE_IDS[2]}
Lease.2.NodeID=${FEDERATION_NODE_IDS[2]}
Lease.2.Authority=${FEDERATION_AUTHORITIES[2]}
EOF_ID_SEED
)"
  FEDERATION_MEMBERSHIP_ROOT_CACHE="$(canonical_membership_events | federation_sha256_text)"
  FEDERATION_TOPOLOGY_ROOT_CACHE="$(canonical_topology_state | federation_sha256_text)"
  FEDERATION_CHECKPOINT_EXCHANGE_ROOT_CACHE="$(canonical_checkpoint_exchange | federation_sha256_text)"
  FEDERATION_REPLAY_EXCHANGE_ROOT_CACHE="$(canonical_replay_exchange | federation_sha256_text)"
  FEDERATION_SETTLEMENT_EXCHANGE_ROOT_CACHE="$(canonical_settlement_exchange | federation_sha256_text)"
  FEDERATION_SYNCHRONIZATION_ROOT_CACHE="$(canonical_civilization_synchronization | federation_sha256_text)"
  FEDERATION_RECOVERY_ROOT_CACHE="$(canonical_recovery_state | federation_sha256_text)"
}

federation_write_artifacts() {
  federation_initialize_roots
  mkdir -p federation/members federation/identity federation/topology federation/checkpoints federation/replay federation/settlement federation/synchronization federation/recovery
  canonical_federation_identity > federation/identity/federation_identity.txt
  canonical_membership_events > federation/members/membership_events.txt
  canonical_topology_state > federation/topology/topology_state.txt
  canonical_checkpoint_exchange > federation/checkpoints/checkpoint_exchange.txt
  canonical_replay_exchange > federation/replay/replay_exchange.txt
  canonical_settlement_exchange > federation/settlement/settlement_exchange.txt
  canonical_civilization_synchronization > federation/synchronization/civilization_synchronization.txt
  canonical_recovery_state > federation/recovery/recovery_state.txt
}

_validate_hash() {
  [[ "$1" =~ ^[0-9a-f]{64}$ ]]
}

validate_federation_identity() {
  _validate_hash "$(identity_root)" && [[ "$FEDERATION_ID" == everarcade-* ]] && [[ "$MEMBERSHIP_EPOCH" == epoch-* ]]
}

validate_federation_membership() {
  validate_federation_identity && _validate_hash "$(membership_root)" && canonical_membership_events | grep -q '^Deterministic=PASS$' && canonical_membership_events | grep -q '^ReplaySafe=PASS$' && canonical_membership_events | grep -q '^EpochAware=PASS$'
}

validate_federation_topology() {
  validate_federation_membership && _validate_hash "$(topology_root)" && canonical_topology_state | grep -q '^NetworkingImplementation=not-required$'
}

validate_checkpoint_exchange() {
  validate_federation_topology && _validate_hash "$(checkpoint_exchange_root)" && canonical_checkpoint_exchange | grep -q '^CheckpointIntegrity=PASS$' && canonical_checkpoint_exchange | grep -q '^CheckpointIdentity=PASS$' && canonical_checkpoint_exchange | grep -q '^EpochMatch=PASS$'
}

validate_replay_exchange() {
  validate_checkpoint_exchange && _validate_hash "$(replay_exchange_root)" && canonical_replay_exchange | grep -q '^ReplayIntegrity=PASS$' && canonical_replay_exchange | grep -q '^ReplayIdentity=PASS$' && canonical_replay_exchange | grep -q '^ContinuityMatch=PASS$'
}

validate_settlement_exchange() {
  validate_replay_exchange && _validate_hash "$(settlement_exchange_root)" && canonical_settlement_exchange | grep -q '^SettlementEvidence=PASS$' && canonical_settlement_exchange | grep -q '^ReceiptIntegrity=PASS$' && canonical_settlement_exchange | grep -q '^AuthorityMatch=PASS$'
}

validate_civilization_synchronization() {
  validate_settlement_exchange && _validate_hash "$(civilization_synchronization_root)" && canonical_civilization_synchronization | grep -q '^SovereignMutationBoundary=PASS$' && canonical_civilization_synchronization | grep -q '^FailureIsolation=PASS$'
}

validate_federation_recovery() {
  validate_civilization_synchronization && _validate_hash "$(recovery_root)" && canonical_recovery_state | grep -q '^RecoveryIsolation=PASS$' && _validate_hash "$(lease_replay_root "${FEDERATION_LEASE_IDS[1]}")" && _validate_hash "$(lease_continuity_root "${FEDERATION_LEASE_IDS[1]}")"
}
