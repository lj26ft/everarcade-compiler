#![allow(dead_code, unused_imports)]
pub mod bootstrap;
pub mod config;
pub mod health;
pub mod lifecycle;
pub mod metrics;
pub mod recovery;
pub mod restart;
pub mod runtime;
pub mod runtime_report;
pub mod scheduler;
pub mod shutdown;
pub mod state;
pub mod supervisor;
pub mod watchdog;
pub mod service_runtime;
pub use self::service_runtime::*;
pub mod service_loop;
pub use self::service_loop::*;
pub mod service_state;
pub use self::service_state::*;
pub mod service_session;
pub use self::service_session::*;
pub mod service_checkpoint;
pub use self::service_checkpoint::*;
pub mod service_recovery;
pub use self::service_recovery::*;
pub mod service_supervisor;
pub use self::service_supervisor::*;
pub mod service_health;
pub use self::service_health::*;
pub mod daemon;
pub mod service;
pub mod status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignRuntimeNodeDaemon {
    pub tcp_listener_active: bool,
    pub websocket_observer_active: bool,
    pub storage_restorable: bool,
    pub non_authoritative: bool,
}

impl SovereignRuntimeNodeDaemon {
    pub fn bootstrap() -> Self {
        Self {
            tcp_listener_active: true,
            websocket_observer_active: true,
            storage_restorable: true,
            non_authoritative: true,
        }
    }

    pub fn readiness(&self) -> bool {
        self.tcp_listener_active
            && self.websocket_observer_active
            && self.storage_restorable
            && self.non_authoritative
    }
}
