use execution_core::scheduler::{
    cadence::TickCadence,
    events::ScheduledEvent,
    executor::DeterministicExecutor,
    queue::DeterministicQueue,
    tick::DeterministicTick,
    window::TickWindow,
    world::{DeterministicWorld, TickReceipt, WorldCheckpoint},
    SchedulerRuntime,
};

#[derive(Clone, Debug)]
struct MockWorld {
    lineage: u64,
    checkpoint: WorldCheckpoint,
    applied_ticks: Vec<DeterministicTick>,
    applied_event_sequences: Vec<Option<u64>>,
    persisted: Vec<WorldCheckpoint>,
}

impl MockWorld {
    fn new(lineage: u64, tick: DeterministicTick) -> Self {
        Self {
            lineage,
            checkpoint: WorldCheckpoint { lineage, tick },
            applied_ticks: Vec::new(),
            applied_event_sequences: Vec::new(),
            persisted: Vec::new(),
        }
    }
}

impl DeterministicWorld for MockWorld {
    fn checkpoint(&self) -> WorldCheckpoint {
        self.checkpoint.clone()
    }

    fn apply(&mut self, tick: DeterministicTick, event: Option<&ScheduledEvent>) -> TickReceipt {
        self.applied_ticks.push(tick);
        self.applied_event_sequences.push(event.map(|e| e.sequence));
        TickReceipt {
            lineage: self.lineage,
            tick,
            event_sequence: event.map(|e| e.sequence),
        }
    }

    fn persist_checkpoint(&mut self, checkpoint: WorldCheckpoint) {
        self.checkpoint = checkpoint.clone();
        self.persisted.push(checkpoint);
    }
}

fn evt(sequence: u64, source: &str, payload: &[u8]) -> ScheduledEvent {
    ScheduledEvent {
        sequence,
        source: source.to_string(),
        payload: payload.to_vec(),
    }
}

#[test]
fn test_event_order_is_deterministic() {
    let mut events = vec![
        evt(2, "z", b"p2"),
        evt(1, "b", b"p1"),
        evt(1, "a", b"p1"),
        evt(1, "a", b"p0"),
    ];
    events.sort();

    let ordered: Vec<(u64, String, Vec<u8>)> = events
        .into_iter()
        .map(|e| (e.sequence, e.source, e.payload))
        .collect();

    assert_eq!(
        ordered,
        vec![
            (1, "a".to_string(), b"p0".to_vec()),
            (1, "a".to_string(), b"p1".to_vec()),
            (1, "b".to_string(), b"p1".to_vec()),
            (2, "z".to_string(), b"p2".to_vec()),
        ]
    );
}

#[test]
fn test_queue_orders_events_canonically() {
    let mut queue = DeterministicQueue::default();
    assert!(queue.push(evt(2, "z", b"p2")));
    assert!(queue.push(evt(1, "b", b"p1")));
    assert!(queue.push(evt(1, "a", b"p1")));
    assert!(queue.push(evt(1, "a", b"p0")));

    let popped: Vec<(u64, String, Vec<u8>)> = std::iter::from_fn(|| queue.pop_next())
        .map(|e| (e.sequence, e.source, e.payload))
        .collect();

    assert_eq!(
        popped,
        vec![
            (1, "a".to_string(), b"p0".to_vec()),
            (1, "a".to_string(), b"p1".to_vec()),
            (1, "b".to_string(), b"p1".to_vec()),
            (2, "z".to_string(), b"p2".to_vec()),
        ]
    );
}

#[test]
fn test_tick_id_is_deterministic() {
    let tick = DeterministicTick(41);
    assert_eq!(tick.next(), DeterministicTick(42));
    assert_eq!(tick.next().next(), DeterministicTick(43));
}

#[test]
fn test_cadence_window_accepts_valid_tick() {
    let cadence = TickCadence {
        ticks_per_window: 4,
    }
    .validate()
    .expect("cadence should validate");

    assert!(cadence.in_same_window(DeterministicTick(8), DeterministicTick(11)));

    let window = TickWindow::from_tick(cadence, DeterministicTick(11));
    assert_eq!(window.start, DeterministicTick(8));
    assert_eq!(window.end, DeterministicTick(11));
}

#[test]
fn test_cadence_window_rejects_invalid_tick() {
    let cadence = TickCadence {
        ticks_per_window: 0,
    };

    assert!(cadence.validate().is_err());
}

#[test]
fn test_executor_advances_one_tick() {
    let world = MockWorld::new(7, DeterministicTick(0));
    let mut executor = DeterministicExecutor::new(world);

    let receipt = executor.execute_tick(DeterministicTick(5), None);
    assert_eq!(receipt.tick, DeterministicTick(5));
    assert_eq!(receipt.event_sequence, None);

    let world_ref = executor.world();
    assert_eq!(world_ref.applied_ticks, vec![DeterministicTick(5)]);
}

#[test]
fn test_executor_emits_receipt() {
    let world = MockWorld::new(11, DeterministicTick(0));
    let mut executor = DeterministicExecutor::new(world);
    let event = evt(99, "source", b"payload");

    let receipt = executor.execute_tick(DeterministicTick(12), Some(&event));
    assert_eq!(receipt.lineage, 11);
    assert_eq!(receipt.tick, DeterministicTick(12));
    assert_eq!(receipt.event_sequence, Some(99));
}

#[test]
fn test_executor_advances_checkpoint_root() {
    let world = MockWorld::new(3, DeterministicTick(0));
    let mut executor = DeterministicExecutor::new(world);

    let _ = executor.execute_tick(DeterministicTick(22), None);

    let world_ref = executor.world();
    let checkpoint = world_ref.checkpoint();
    assert_eq!(checkpoint.lineage, 3);
    assert_eq!(checkpoint.tick, DeterministicTick(22));
    assert_eq!(world_ref.persisted.len(), 1);
}

#[test]
fn test_runtime_loop_processes_queue_in_order() {
    let world = MockWorld::new(42, DeterministicTick(0));
    let mut queue = DeterministicQueue::default();
    assert!(queue.push(evt(2, "z", b"p2")));
    assert!(queue.push(evt(1, "a", b"p0")));
    assert!(queue.push(evt(1, "a", b"p1")));

    let mut runtime = SchedulerRuntime::new(world, queue, DeterministicTick(100));

    let r1 = runtime.run_one_tick();
    let r2 = runtime.run_one_tick();
    let r3 = runtime.run_one_tick();

    assert_eq!(r1.tick, DeterministicTick(100));
    assert_eq!(r2.tick, DeterministicTick(101));
    assert_eq!(r3.tick, DeterministicTick(102));
    assert_eq!(r1.event_sequence, Some(1));
    assert_eq!(r2.event_sequence, Some(1));
    assert_eq!(r3.event_sequence, Some(2));
}

#[test]
fn test_runtime_loop_is_replay_stable() {
    fn run_once() -> Vec<(u64, u64, Option<u64>)> {
        let world = MockWorld::new(9, DeterministicTick(0));
        let mut queue = DeterministicQueue::default();
        assert!(queue.push(evt(3, "c", b"3")));
        assert!(queue.push(evt(1, "a", b"1")));
        assert!(queue.push(evt(2, "b", b"2")));

        let mut runtime = SchedulerRuntime::new(world, queue, DeterministicTick(50));
        (0..3)
            .map(|_| runtime.run_one_tick())
            .map(|r| (r.lineage, r.tick.0, r.event_sequence))
            .collect()
    }

    let first = run_once();
    let second = run_once();
    assert_eq!(first, second);
}
