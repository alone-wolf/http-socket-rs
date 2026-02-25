use crate::protocol::capability::{CapabilityMap, CapabilitySet};
use crate::protocol::version::ProtocolVersion;
use crate::transport::types::TransportKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientAdvertise {
    pub transports: Vec<TransportKind>,
    pub versions: Vec<ProtocolVersion>,
    pub capabilities: CapabilityMap,
    pub required_capabilities: CapabilitySet,
}

impl ClientAdvertise {
    pub fn new(
        transports: Vec<TransportKind>,
        versions: Vec<ProtocolVersion>,
        capabilities: CapabilityMap,
        required_capabilities: CapabilitySet,
    ) -> Self {
        Self {
            transports,
            versions,
            capabilities,
            required_capabilities,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerSelect {
    pub transport: TransportKind,
    pub version: ProtocolVersion,
    pub enabled_capabilities: CapabilityMap,
}
