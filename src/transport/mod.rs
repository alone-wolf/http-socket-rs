//! Transport abstractions and feature-gated transport handle types.

pub mod handle;
pub mod registry;
pub mod types;

#[cfg(feature = "poll")]
pub mod poll;

#[cfg(feature = "sse")]
pub mod sse;

#[cfg(feature = "ws")]
pub mod ws;
