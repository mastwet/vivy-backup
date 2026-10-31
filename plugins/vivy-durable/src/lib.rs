//! Experimental probes, selected explicitly at compilation. No durable runtime
//! or disk-sync guarantee is supplied by this crate.

#[cfg(feature = "d0-probe")]
mod probe;
#[cfg(feature = "d0-probe")]
pub use probe::D0ProbePlugin;
