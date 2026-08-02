use std::fmt;

use busylib::model::ble::{BleState, BleStatusResponse};
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::fields::{field, write_fields};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BleStatusEvent(BleStatusResponse);

impl BleStatusEvent {
    pub fn new(status: BleStatusResponse) -> Self {
        Self(status)
    }
}

impl From<BleStatusEvent> for CliEvent {
    fn from(event: BleStatusEvent) -> Self {
        CliEvent::BleStatus(event)
    }
}

impl fmt::Display for BleStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = &self.0;
        let mut fields = vec![field("status", ble_state_label(&status.status))];

        if let Some(address) = &status.address {
            fields.push(field("address", address));
        }

        write_fields(f, &fields)
    }
}

fn ble_state_label(state: &BleState) -> &str {
    match state {
        BleState::Reset => "reset",
        BleState::Initialization => "initialization",
        BleState::Disabled => "disabled",
        BleState::Enabled => "enabled",
        BleState::Connectable => "connectable",
        BleState::Connected => "connected",
        BleState::InternalError => "internal error",
        BleState::Unknown(state) => state,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn renders_the_address_only_when_connected() {
        let connected = BleStatusEvent::new(BleStatusResponse {
            status: BleState::Connected,
            address: Some("E8:D5:2B:0D:9B:20".to_string()),
        });
        let disabled = BleStatusEvent::new(BleStatusResponse {
            status: BleState::Disabled,
            address: None,
        });

        assert_eq!(
            connected.to_string(),
            "status: connected\naddress: E8:D5:2B:0D:9B:20"
        );
        assert_eq!(disabled.to_string(), "status: disabled");
    }

    #[test]
    fn keeps_the_space_in_the_internal_error_state() {
        let event = BleStatusEvent::new(BleStatusResponse {
            status: BleState::InternalError,
            address: None,
        });

        assert_eq!(event.to_string(), "status: internal error");
        assert_eq!(
            serde_json::to_value(CliEvent::from(event)).unwrap(),
            json!({"event": "ble_status", "status": "internal error", "address": null})
        );
    }
}
