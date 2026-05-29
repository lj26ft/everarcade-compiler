pub fn scrub_timeline(frames: &[&str], cursor: usize) -> String { let frame = frames.get(cursor).copied().unwrap_or("end"); crate::stable_hash(&["timeline", frame, &cursor.to_string()]) }
