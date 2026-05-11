#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledPackage {
    pub package_id: String,
    pub epoch: u64,
}

#[derive(Debug, Default)]
pub struct Scheduler;

impl Scheduler {
    pub fn schedule(mut packages: Vec<ScheduledPackage>) -> Vec<ScheduledPackage> {
        packages.sort_by(|a, b| a.epoch.cmp(&b.epoch).then(a.package_id.cmp(&b.package_id)));
        packages
    }
}
