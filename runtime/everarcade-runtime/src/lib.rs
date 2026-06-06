//! Minimal EverArcade appliance runtime.
//!
//! Pseudocode architecture:
//! - Authority: the runtime package manifest, signed-by-hash package bytes, journal, receipts,
//!   and checkpoints are the local authoritative evidence for this appliance process.
//! - Non-authoritative: renderer, player-gateway, creator-template, GPU, XRPL, and Xaman records
//!   are consumers or external-boundary evidence; this crate does not make those systems true.
//! - Input: an operator command plus a runtime root, world id, and package directory.
//! - Output: runtime status, journal entries, receipts, checkpoints, backups, and replay reports.
//! - Fit: this is the small runnable core under many larger EverArcade scaffold directories.

pub mod runtime;

pub use runtime::*;
