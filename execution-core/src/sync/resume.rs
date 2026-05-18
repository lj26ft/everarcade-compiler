use std::path::Path;

use crate::sync::{
    cursor::SyncCursor,
    history::{append_cursor_history, CursorHistoryEntry},
    observer::{advance_observer, verify_observer_state, ObserverState},
    persistence::{load_observer_state, save_observer_state},
    rollback::detect_rollback,
};

pub fn resume_from_cursor(
    world_root: &Path,
    incoming: &[SyncCursor],
    history: &mut Vec<CursorHistoryEntry>,
) -> Result<(u64, u64, ObserverState), String> {
    let mut state = load_observer_state(world_root)?;
    verify_observer_state(&state)?;
    let mut window_start = state.highest_verified_sequence + 1;
    let mut window_end = state.highest_verified_sequence;

    for cursor in incoming {
        if cursor.latest_sequence <= state.highest_verified_sequence {
            continue;
        }
        let rollback = detect_rollback(&state.current_cursor, cursor, history);
        if rollback.rollback_detected {
            return Err(format!(
                "rollback detected expected={} actual={}",
                rollback.expected_sequence, rollback.actual_sequence
            ));
        }
        if window_end == state.highest_verified_sequence {
            window_start = cursor.latest_sequence;
        }
        advance_observer(&mut state, cursor.clone(), true)?;
        append_cursor_history(
            history,
            CursorHistoryEntry {
                sequence: cursor.latest_sequence,
                checkpoint_root: cursor.latest_checkpoint_root,
                execution_id: cursor.latest_execution_id,
            },
        )?;
        window_end = cursor.latest_sequence;
    }
    save_observer_state(world_root, &state)?;
    Ok((window_start, window_end, state))
}
