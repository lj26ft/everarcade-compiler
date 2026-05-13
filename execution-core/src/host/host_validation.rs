use super::host_state::HostStateRecord;

pub fn validate_host_state_record(record: &HostStateRecord) -> bool {
    record.commit_root != [0; 32]
}
