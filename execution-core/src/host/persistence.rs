use super::host_state::HostStateRecord;

pub trait HostStatePersistence {
    fn persist(&self, _record: &HostStateRecord) -> bool;
}
