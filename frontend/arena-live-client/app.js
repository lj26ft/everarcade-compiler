let sequence = 0;
const $ = (id) => document.getElementById(id);
const runtimeUrl = () => $('runtimeUrl').value.replace(/\/$/, '');
const playerId = () => $('playerId').value.trim() || 'player-a';
function renderJson(id, value) { $(id).textContent = JSON.stringify(value, null, 2); }
async function refreshState() {
  const state = await fetch(`${runtimeUrl()}/state`).then((response) => response.json());
  const players = Object.values(state.players || {});
  renderJson('players', players.map((player) => ({ id: player.id, x: player.x, y: player.y })));
  renderJson('health', Object.fromEntries(players.map((player) => [player.id, player.health])));
  renderJson('score', Object.fromEntries(players.map((player) => [player.id, player.score])));
  renderJson('combatEvents', state.combat_events || []);
  $('tick').textContent = state.tick ?? 0;
  $('stateRoot').textContent = state.state_root || 'unknown';
  $('replayStatus').textContent = state.replay_status || 'unknown';
  sequence = Math.max(sequence, state.last_sequence?.[playerId()] || 0);
}
async function action(actionType, extra = {}) {
  sequence += 1;
  const payload = actionType === 'attack' ? { action: actionType, player: playerId(), target: extra.target || 'player-2' } : { action: actionType, player: playerId(), ...extra };
  const response = await fetch(`${runtimeUrl()}/input`, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(payload) });
  const receipt = await response.json();
  renderJson('receipt', receipt);
  await refreshState();
}
$('joinBtn').addEventListener('click', () => action('join').catch((error) => renderJson('receipt', { error: String(error) })));
$('moveBtn').addEventListener('click', () => action('move', { direction: 'north' }).catch((error) => renderJson('receipt', { error: String(error) })));
$('attackBtn').addEventListener('click', () => action('attack').catch((error) => renderJson('receipt', { error: String(error) })));
$('refreshBtn').addEventListener('click', () => refreshState().catch((error) => renderJson('receipt', { error: String(error) })));
refreshState().catch(() => {});
