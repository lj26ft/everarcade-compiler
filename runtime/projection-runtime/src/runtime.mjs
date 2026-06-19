import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';

export const PROJECTION_VERSION = '0.1.0';

export function cloneArenaState(arenaState) {
  return JSON.parse(JSON.stringify(arenaState));
}

export function canonicalize(value) {
  if (Array.isArray(value)) return value.map(canonicalize);
  if (value && typeof value === 'object') {
    return Object.fromEntries(Object.keys(value).sort().map((key) => [key, canonicalize(value[key])]));
  }
  return value;
}

export function stateRoot(arenaState) {
  return createHash('sha256').update(JSON.stringify(canonicalize(arenaState))).digest('hex');
}

export function projectArenaState(arenaState, { operator = 'Operator A', mode = 'live' } = {}) {
  const state = cloneArenaState(arenaState);
  const receipts = state.receipts ?? [];
  const proposals = state.governance?.proposals ?? [];
  return Object.freeze({
    projectionVersion: PROJECTION_VERSION,
    operator,
    mode,
    worldName: state.world?.name ?? 'Unnamed Arena',
    epoch: state.continuity?.epoch ?? 0,
    tick: state.continuity?.tick ?? 0,
    root: stateRoot(state),
    players: Object.freeze((state.players ?? []).map((player) => Object.freeze({
      player_id: player.player_id,
      score: player.score ?? 0,
      status: player.status ?? 'unknown',
      inventory_count: (player.inventory ?? []).length,
    }))),
    entities: Object.freeze((state.entities ?? []).map((entity) => Object.freeze({
      entity_id: entity.entity_id,
      entity_type: entity.entity_type,
    }))),
    positions: Object.freeze((state.positions ?? []).map((position) => Object.freeze({
      entity_id: position.entity_id,
      x: position.x,
      y: position.y,
    }))),
    health: Object.freeze((state.health ?? []).map((health) => Object.freeze({
      entity_id: health.entity_id,
      health: health.health,
      max_health: health.max_health,
    }))),
    receipt_count: receipts.length,
    latest_receipt_count: receipts.length,
    market_receipts: Object.freeze(receipts.filter((receipt) => receipt.domain === 'market').map((receipt) => Object.freeze(receipt))),
    governance: Object.freeze(proposals.map((proposal) => Object.freeze({
      proposal_id: proposal.proposal_id,
      status: proposal.status,
      yes: proposal.yes ?? 0,
      no: proposal.no ?? 0,
    }))),
  });
}

export function loadJournal(path) {
  const journal = JSON.parse(readFileSync(path, 'utf8'));
  if (!Array.isArray(journal.ticks)) throw new Error('journal.ticks must be an array');
  return journal;
}

export function replayJournal(journal, options = {}) {
  return journal.ticks.map((entry) => projectArenaState(entry.arenaState, { ...options, mode: 'replay' }));
}

export function assertProjectionReadOnly(before, after) {
  const beforeRoot = stateRoot(before);
  const afterRoot = stateRoot(after);
  if (beforeRoot !== afterRoot) {
    throw new Error(`Projection mutated ArenaState: ${beforeRoot} != ${afterRoot}`);
  }
  return true;
}

export function compareVisuals(left, right) {
  return JSON.stringify(canonicalize(left)) === JSON.stringify(canonicalize(right));
}
