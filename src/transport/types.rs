use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransportKind {
    Ws,
    Sse,
    Poll,
    Quic,
    WebTransport,
}

impl fmt::Display for TransportKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            TransportKind::Ws => "ws",
            TransportKind::Sse => "sse",
            TransportKind::Poll => "poll",
            TransportKind::Quic => "quic",
            TransportKind::WebTransport => "web_transport",
        };

        write!(f, "{}", value)
    }
}
