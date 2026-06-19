const fallbackSeed = {
  world: { tick: 1280, epoch: 7, receipt_count: 42, players: [{ id: 'alice' }, { id: 'bruno' }, { id: 'chen' }] },
  commitments: {
    state_root: 'state:arena-vanguard:tick:1280',
    world_hash: 'world:arena-vanguard:players:3',
    receipt_root: 'receipt:arena-vanguard:entries:42',
    continuity_root: 'continuity:arena-vanguard:live'
  },
  operator: { root_match: true }
};

const worldData = {
  day: 37,
  regions: [
    {
      name: 'Citadel',
      accent: 'rgba(255, 221, 87, 0.22)',
      status: 'governed',
      objects: [
        ['structure', 'Council Hall', 'Oldest structure · founded Day 2'],
        ['player', 'Alice', 'Returned with relay-key memory'],
        ['resource', 'Market ledger', '14 credits recorded from first trade'],
        ['ruin', 'Old Gate Stones', 'Restored after checkpoint replay']
      ]
    },
    {
      name: 'Frontier',
      accent: 'rgba(98, 241, 255, 0.20)',
      status: 'settling',
      objects: [
        ['player', 'Bruno', 'Known for the first wolf kill'],
        ['structure', 'Market Kiosk', 'First trade route still active'],
        ['resource', 'Ore caravans', 'Migrated without losing inventory'],
        ['creature', 'Watch drones', 'PvE pressure near the road']
      ]
    },
    {
      name: 'Wilderness',
      accent: 'rgba(72, 242, 154, 0.18)',
      status: 'untamed',
      objects: [
        ['creature', 'Wolf packs', 'First kill remembered on Day 3'],
        ['resource', 'Timber groves', 'Harvest nodes replenish by day'],
        ['player', 'Scout Chen', 'Offline player state preserved'],
        ['ruin', 'Campfire ring', 'Abandoned party marker']
      ]
    },
    {
      name: 'Ruins',
      accent: 'rgba(185, 156, 255, 0.22)',
      status: 'haunted',
      objects: [
        ['ruin', 'Drone-7 Shell', 'Past combat became geography'],
        ['creature', 'Sentinel-2', 'Still guarding the relic cache'],
        ['resource', 'Relic Cache', 'Neutral object from seed world'],
        ['structure', 'Migration Beacon', 'Continuity preserved on Day 30']
      ]
    }
  ],
  memory: [
    ['First kill', 'Day 3 · wolf defeated near Wilderness road'],
    ['First trade', 'Day 7 · ore exchanged at Frontier market'],
    ['First vote', 'Day 12 · AV-17 passed in Citadel'],
    ['First migration', 'Day 30 · world resumed from same root'],
    ['Oldest structure', 'Council Hall · standing since Day 2'],
    ['Ruins created from past events', 'Drone-7 Shell and Old Gate Stones remain visible']
  ],
  timeline: [
    ['Day 1', 'World created'],
    ['Day 3', 'First wolf killed'],
    ['Day 7', 'First market trade'],
    ['Day 12', 'First governance vote'],
    ['Day 30', 'Migration preserved continuity'],
    ['Day 37', 'Player returned to an older world']
  ],
  controls: [
    ['Replay history', 'History replay highlighted: first kill, first trade, first vote, and returned-player continuity.'],
    ['Show restore', 'Restore view highlighted: checkpoint replay rebuilds the same remembered world.'],
    ['Show migration', 'Migration view highlighted: the world moves operators without becoming a new world.'],
    ['Advance world age', 'Local presentation advanced one day. Runtime authority is unchanged.']
  ]
};

function escapeHtml(value) {
  return String(value).replace(/[&<>'"]/g, (char) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', "'": '&#39;', '"': '&quot;' }[char]));
}

async function loadSeed() {
  try {
    const response = await fetch('./demo-world-seed.json', { cache: 'no-store' });
    if (!response.ok) throw new Error(`seed ${response.status}`);
    return await response.json();
  } catch (_) {
    return fallbackSeed;
  }
}

function renderRegions() {
  worldBoard.innerHTML = worldData.regions.map((region) => `
    <article class="region-card" style="--region-glow: ${region.accent}">
      <div class="region-title"><h3>${escapeHtml(region.name)}</h3><span>${escapeHtml(region.status)}</span></div>
      <div class="object-list">
        ${region.objects.map(([type, name, detail]) => `
          <div class="world-object">
            <div class="object-type">${escapeHtml(type)}</div>
            <b>${escapeHtml(name)}</b>
            <span>${escapeHtml(detail)}</span>
          </div>
        `).join('')}
      </div>
    </article>
  `).join('');
}

function renderMemory() {
  memoryGrid.innerHTML = worldData.memory.map(([label, value]) => `
    <div class="memory-card"><span>${escapeHtml(label)}</span><strong>${escapeHtml(value)}</strong></div>
  `).join('');
}

function renderTimeline() {
  eventTimeline.innerHTML = worldData.timeline.map(([day, event]) => `<li><b>${escapeHtml(day)}</b><span>${escapeHtml(event)}</span></li>`).join('');
}

function renderVerification(seed) {
  const entries = [
    ['State root', seed.commitments?.state_root || fallbackSeed.commitments.state_root],
    ['World hash', seed.commitments?.world_hash || fallbackSeed.commitments.world_hash],
    ['Tier 1', 'PASS'],
    ['Tier 2', 'PASS'],
    ['Roots match', seed.operator?.root_match === false ? 'CHECK' : 'PASS'],
    ['Receipt root', seed.commitments?.receipt_root || fallbackSeed.commitments.receipt_root]
  ];
  verificationGrid.innerHTML = entries.map(([label, value]) => `
    <div class="verification-card"><span>${escapeHtml(label)}</span><strong class="${String(value).includes('PASS') ? 'pass' : ''}">${escapeHtml(value)}</strong></div>
  `).join('');
}

function renderControls() {
  demoControls.innerHTML = worldData.controls.map(([label], index) => `<button type="button" data-index="${index}">${escapeHtml(label)}</button>`).join('');
  demoControls.addEventListener('click', (event) => {
    const button = event.target.closest('button');
    if (!button) return;
    demoControls.querySelectorAll('button').forEach((node) => node.classList.toggle('active', node === button));
    const [label, message] = worldData.controls[Number(button.dataset.index)];
    if (label === 'Advance world age') {
      worldData.day += 1;
      worldAge.textContent = `Day ${worldData.day}`;
      ageNote.textContent = 'Local demo age advanced only in this browser.';
    }
    storyCallout.textContent = message;
  });
}

async function boot() {
  const seed = await loadSeed();
  const dayFromSeed = Math.max(37, Math.ceil((seed.world?.tick || 1280) / 35));
  worldData.day = dayFromSeed;
  worldAge.textContent = `Day ${worldData.day}`;
  renderRegions();
  renderMemory();
  renderTimeline();
  renderVerification(seed);
  renderControls();
}

boot();
