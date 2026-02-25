use crate::error::NegotiationError;
use crate::protocol::capability::{CapabilityMap, CapabilitySet};
use crate::protocol::handshake::{ClientAdvertise, NegotiationPolicy, ServerSelect};
use crate::protocol::version::ProtocolVersion;
use crate::transport::types::TransportKind;

pub fn negotiate(
    advertise: &ClientAdvertise,
    policy: &dyn NegotiationPolicy,
) -> Result<ServerSelect, NegotiationError> {
    let version = select_version(advertise.versions.as_slice(), policy.version_preference())?;
    let transport = select_transport(
        advertise.transports.as_slice(),
        policy.transport_preference(),
    )?;
    let enabled_capabilities = select_capabilities(
        &advertise.capabilities,
        &advertise.required_capabilities,
        policy.required_capabilities(),
        policy,
    )?;

    Ok(ServerSelect {
        transport,
        version,
        enabled_capabilities,
    })
}

pub fn select_version(
    client_versions: &[ProtocolVersion],
    server_preferred_versions: &[ProtocolVersion],
) -> Result<ProtocolVersion, NegotiationError> {
    if let Some(version) = server_preferred_versions
        .iter()
        .copied()
        .find(|version| client_versions.contains(version))
    {
        return Ok(version);
    }

    Err(NegotiationError::NoVersionIntersection)
}

pub fn select_transport(
    client_transports: &[TransportKind],
    server_preferred_transports: &[TransportKind],
) -> Result<TransportKind, NegotiationError> {
    if let Some(transport) = server_preferred_transports
        .iter()
        .copied()
        .find(|transport| client_transports.contains(transport))
    {
        return Ok(transport);
    }

    Err(NegotiationError::NoTransportIntersection)
}

pub fn select_capabilities(
    client_capabilities: &CapabilityMap,
    client_required: &CapabilitySet,
    server_required: &CapabilitySet,
    policy: &dyn NegotiationPolicy,
) -> Result<CapabilityMap, NegotiationError> {
    let enabled = policy.filter_server_capabilities(client_capabilities);

    for required in client_required {
        if !enabled.contains_key(required) {
            return Err(NegotiationError::MissingRequiredCapability(
                required.to_string(),
            ));
        }
    }

    for required in server_required {
        if !enabled.contains_key(required) {
            return Err(NegotiationError::MissingRequiredCapability(
                required.to_string(),
            ));
        }
    }

    Ok(enabled)
}
