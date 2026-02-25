use std::sync::Arc;

use crate::core::session::core::SessionCore;
use crate::core::state::SessionState;
use crate::error::{FrameworkError, StateError};
use crate::protocol::handshake::CapabilityContract;
use crate::transport::handle::TransportHandle;

pub fn attach_transport(
    session: &mut SessionCore,
    contract: CapabilityContract,
    transport: Arc<dyn TransportHandle>,
) -> Result<(), FrameworkError> {
    if session.transport().is_some() {
        return Err(FrameworkError::State(StateError::TransportAlreadyAttached));
    }

    if session.state() == SessionState::Closed {
        return Err(FrameworkError::State(StateError::SessionClosed));
    }

    session.transition_to(SessionState::Negotiating)?;
    session.set_contract(contract);
    session.set_transport(transport);
    session.transition_to(SessionState::Active)
}
