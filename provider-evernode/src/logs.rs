use control_plane::logs::{LogEntry, LogKind, LogStore};

pub fn append_provider_log(store: &mut LogStore, timestamp_ms: u64, kind: LogKind, subject: &str) {
    store.append(
        timestamp_ms,
        kind,
        subject,
        vec![("provider".into(), "evernode".into())],
    );
}

pub fn searchable_logs(store: &LogStore, term: &str) -> Vec<LogEntry> {
    store.search(term).into_iter().cloned().collect()
}
