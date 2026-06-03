use crate::runtime::{RuntimeConfiguration, RuntimeLoop};
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct RuntimeSupervisor {
    pub config: RuntimeConfiguration,
    pub restart_count: u64,
}

impl RuntimeSupervisor {
    pub fn new(config: RuntimeConfiguration) -> Self {
        Self {
            config,
            restart_count: 0,
        }
    }
    pub fn start(&mut self) -> Result<RuntimeLoop> {
        RuntimeLoop::boot(self.config.clone())
    }
    pub fn restart(&mut self) -> Result<RuntimeLoop> {
        self.restart_count += 1;
        RuntimeLoop::boot(self.config.clone())
    }
}
