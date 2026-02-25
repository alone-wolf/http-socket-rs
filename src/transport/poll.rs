//! Lightweight long-polling transport handle used by the session runtime.

use std::sync::atomic::{AtomicU64, Ordering};

use crate::transport::handle::TransportHandle;
use crate::transport::types::TransportKind;

static POLL_TRANSPORT_ID_ALLOC: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
/// Feature-gated transport handle tagged as `poll`.
pub struct PollTransport {
    id: u64,
    open: bool,
}

impl PollTransport {
    pub fn new() -> Self {
        let id = POLL_TRANSPORT_ID_ALLOC.fetch_add(1, Ordering::Relaxed);
        Self { id, open: true }
    }

    pub fn with_id(id: u64) -> Self {
        Self { id, open: true }
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}

impl Default for PollTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl TransportHandle for PollTransport {
    fn id(&self) -> u64 {
        self.id
    }

    fn kind(&self) -> TransportKind {
        TransportKind::Poll
    }

    fn is_open(&self) -> bool {
        self.open
    }
}
