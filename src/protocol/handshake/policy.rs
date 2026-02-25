use crate::protocol::capability::{CapabilityKey, CapabilityMap, CapabilitySet};
use crate::protocol::version::ProtocolVersion;
use crate::transport::types::TransportKind;

pub trait NegotiationPolicy {
    fn required_capabilities(&self) -> &CapabilitySet;

    fn version_preference(&self) -> &[ProtocolVersion];

    fn transport_preference(&self) -> &[TransportKind];

    fn filter_server_capabilities(&self, client_capabilities: &CapabilityMap) -> CapabilityMap {
        client_capabilities
            .iter()
            .filter_map(|(key, value)| {
                self.allow_capability(key)
                    .then_some((key.clone(), value.clone()))
            })
            .collect()
    }

    fn allow_capability(&self, _key: &CapabilityKey) -> bool {
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct ServerPreferencePolicy {
    pub preferred_versions: Vec<ProtocolVersion>,
    pub preferred_transports: Vec<TransportKind>,
    pub required: CapabilitySet,
}

impl ServerPreferencePolicy {
    pub fn new(
        preferred_versions: Vec<ProtocolVersion>,
        preferred_transports: Vec<TransportKind>,
    ) -> Self {
        Self {
            preferred_versions,
            preferred_transports,
            required: CapabilitySet::default(),
        }
    }

    pub fn with_required_capability(mut self, key: CapabilityKey) -> Self {
        self.required.insert(key);
        self
    }
}

impl NegotiationPolicy for ServerPreferencePolicy {
    fn required_capabilities(&self) -> &CapabilitySet {
        &self.required
    }

    fn version_preference(&self) -> &[ProtocolVersion] {
        &self.preferred_versions
    }

    fn transport_preference(&self) -> &[TransportKind] {
        &self.preferred_transports
    }
}
