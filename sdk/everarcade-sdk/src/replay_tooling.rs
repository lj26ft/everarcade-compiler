use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayEvent {
    pub tick: u64,
    pub entity_id: String,
    pub interaction: String,
    pub partition: String,
    pub governance_event: Option<String>,
    pub economic_delta: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayTimeline {
    pub world_state_roots: Vec<String>,
    pub events: Vec<ReplayEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayDiff {
    pub diverged_ticks: Vec<u64>,
    pub root_mismatches: Vec<(u64, String, String)>,
}

pub fn load_replay_timeline(
    events: Vec<ReplayEvent>,
    world_state_roots: Vec<String>,
) -> ReplayTimeline {
    ReplayTimeline {
        events,
        world_state_roots,
    }
}

pub fn step_replay_tick(timeline: &ReplayTimeline, tick: u64) -> Vec<ReplayEvent> {
    timeline
        .events
        .iter()
        .filter(|e| e.tick == tick)
        .cloned()
        .collect()
}

pub fn inspect_replay_state(timeline: &ReplayTimeline, tick: u64) -> BTreeMap<String, String> {
    let mut state = BTreeMap::new();
    let root = timeline
        .world_state_roots
        .get(tick as usize)
        .cloned()
        .unwrap_or_else(|| "missing-root".into());
    state.insert("tick".into(), tick.to_string());
    state.insert("world_state_root".into(), root);
    state.insert(
        "event_count".into(),
        step_replay_tick(timeline, tick).len().to_string(),
    );
    state
}

pub fn diff_replay_timelines(a: &ReplayTimeline, b: &ReplayTimeline) -> ReplayDiff {
    let max_ticks = a.world_state_roots.len().max(b.world_state_roots.len());
    let mut diverged_ticks = BTreeSet::new();
    let mut root_mismatches = Vec::new();
    for tick in 0..max_ticks {
        let ar = a
            .world_state_roots
            .get(tick)
            .cloned()
            .unwrap_or_else(|| "<missing>".into());
        let br = b
            .world_state_roots
            .get(tick)
            .cloned()
            .unwrap_or_else(|| "<missing>".into());
        if ar != br {
            diverged_ticks.insert(tick as u64);
            root_mismatches.push((tick as u64, ar, br));
        }
    }
    ReplayDiff {
        diverged_ticks: diverged_ticks.into_iter().collect(),
        root_mismatches,
    }
}

pub fn inspect_entity_timeline(timeline: &ReplayTimeline, entity_id: &str) -> Vec<ReplayEvent> {
    timeline
        .events
        .iter()
        .filter(|e| e.entity_id == entity_id)
        .cloned()
        .collect()
}

pub fn replay_entity_evolution(timeline: &ReplayTimeline, entity_id: &str) -> Vec<String> {
    inspect_entity_timeline(timeline, entity_id)
        .into_iter()
        .map(|e| {
            format!(
                "tick={} interaction={} partition={}",
                e.tick, e.interaction, e.partition
            )
        })
        .collect()
}

pub fn trace_entity_interactions(timeline: &ReplayTimeline, entity_id: &str) -> Vec<String> {
    inspect_entity_timeline(timeline, entity_id)
        .into_iter()
        .map(|e| e.interaction)
        .collect()
}

pub fn trace_interaction_event(timeline: &ReplayTimeline, tick: u64) -> Vec<ReplayEvent> {
    step_replay_tick(timeline, tick)
}

pub fn verify_interaction_trace(timeline: &ReplayTimeline, tick: u64) -> bool {
    !trace_interaction_event(timeline, tick).is_empty()
}

pub fn reconstruct_interaction_lineage(timeline: &ReplayTimeline, entity_id: &str) -> Vec<String> {
    trace_entity_interactions(timeline, entity_id)
}

pub fn diff_world_replay(a: &ReplayTimeline, b: &ReplayTimeline) -> ReplayDiff {
    diff_replay_timelines(a, b)
}

pub fn diff_entity_state(a: &ReplayTimeline, b: &ReplayTimeline, entity_id: &str) -> bool {
    inspect_entity_timeline(a, entity_id) == inspect_entity_timeline(b, entity_id)
}

pub fn detect_simulation_divergence(a: &ReplayTimeline, b: &ReplayTimeline) -> bool {
    !diff_replay_timelines(a, b).diverged_ticks.is_empty()
}

pub fn verify_deterministic_contract(source: &str) -> bool {
    [
        "std::time::SystemTime",
        "rand::thread_rng",
        "std::fs::write",
        "std::net::TcpStream",
        "f32",
        "f64",
    ]
    .iter()
    .all(|bad| !source.contains(bad))
}

pub fn detect_nondeterministic_behavior(source: &str) -> Vec<&'static str> {
    let mut out = Vec::new();
    if source.contains("std::time::SystemTime") {
        out.push("wall-clock access");
    }
    if source.contains("rand::thread_rng") {
        out.push("unseeded randomness");
    }
    if source.contains("HashMap") && !source.contains("BTreeMap") {
        out.push("nondeterministic iteration");
    }
    if source.contains("std::fs::") {
        out.push("filesystem mutation");
    }
    if source.contains("std::net::") {
        out.push("network mutation");
    }
    if source.contains("f32") || source.contains("f64") {
        out.push("floating-point drift");
    }
    out
}
