use std::fmt;

use busylib::model::smart_home::{
    PairingStatus, SmartHomePairingInfo, SmartHomePairingPayload, SmartHomeSwitchState,
    SwitchStartup,
};
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::fields::{field, write_fields};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SmartHomePairingEvent(SmartHomePairingInfo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SmartHomeStartPairingEvent(SmartHomePairingPayload);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SmartHomeSwitchEvent(SmartHomeSwitchState);

impl SmartHomePairingEvent {
    pub fn new(pairing: SmartHomePairingInfo) -> Self {
        Self(pairing)
    }
}

impl SmartHomeStartPairingEvent {
    pub fn new(payload: SmartHomePairingPayload) -> Self {
        Self(payload)
    }
}

impl SmartHomeSwitchEvent {
    pub fn new(switch: SmartHomeSwitchState) -> Self {
        Self(switch)
    }
}

impl From<SmartHomePairingEvent> for CliEvent {
    fn from(event: SmartHomePairingEvent) -> Self {
        CliEvent::SmartHomePairing(event)
    }
}

impl From<SmartHomeStartPairingEvent> for CliEvent {
    fn from(event: SmartHomeStartPairingEvent) -> Self {
        CliEvent::SmartHomeStartPairing(event)
    }
}

impl From<SmartHomeSwitchEvent> for CliEvent {
    fn from(event: SmartHomeSwitchEvent) -> Self {
        CliEvent::SmartHomeSwitch(event)
    }
}

impl fmt::Display for SmartHomePairingEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pairing = &self.0;
        let mut fields = Vec::new();

        if let Some(fabric_count) = pairing.fabric_count {
            fields.push(field("fabric count", fabric_count));
        }

        if let Some(latest) = &pairing.latest_pairing_status {
            fields.push(field("latest status", pairing_status_label(&latest.value)));

            if let Some(timestamp) = latest.timestamp {
                fields.push(field("latest status timestamp", timestamp));
            }
        }

        write_fields(f, &fields)
    }
}

impl fmt::Display for SmartHomeStartPairingEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let payload = &self.0;
        let mut fields = Vec::new();

        if let Some(available_until) = payload.available_until {
            fields.push(field("available until", available_until));
        }

        if let Some(qr_code) = &payload.qr_code {
            fields.push(field("qr code", qr_code));
        }

        if let Some(manual_code) = &payload.manual_code {
            fields.push(field("manual code", manual_code));
        }

        write_fields(f, &fields)
    }
}

impl fmt::Display for SmartHomeSwitchEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let switch = &self.0;
        let mut fields = Vec::new();

        if let Some(state) = switch.state {
            fields.push(field("state", if state { "on" } else { "off" }));
        }

        if let Some(startup) = switch.startup {
            fields.push(field("startup", switch_startup_label(startup)));
        }

        write_fields(f, &fields)
    }
}

fn pairing_status_label(status: &PairingStatus) -> &str {
    match status {
        PairingStatus::NeverStarted => "never_started",
        PairingStatus::Started => "started",
        PairingStatus::CompletedSuccessfully => "completed_successfully",
        PairingStatus::Failed => "failed",
        PairingStatus::Unknown(status) => status,
    }
}

fn switch_startup_label(startup: SwitchStartup) -> &'static str {
    match startup {
        SwitchStartup::Off => "off",
        SwitchStartup::On => "on",
        SwitchStartup::Toggle => "toggle",
        SwitchStartup::Last => "last",
    }
}

#[cfg(test)]
mod tests {
    use busylib::model::smart_home::PairingStatusInfo;
    use serde_json::json;

    use super::*;

    #[test]
    fn renders_the_pairing_status_without_a_timestamp() {
        let event = SmartHomePairingEvent::new(SmartHomePairingInfo {
            fabric_count: Some(0),
            latest_pairing_status: Some(PairingStatusInfo {
                value: PairingStatus::NeverStarted,
                timestamp: None,
            }),
        });

        assert_eq!(
            event.to_string(),
            "fabric count: 0\nlatest status: never_started"
        );
    }

    #[test]
    fn renders_the_switch_state_as_on_or_off() {
        let on = SmartHomeSwitchEvent::new(SmartHomeSwitchState::on());
        let off = SmartHomeSwitchEvent::new(SmartHomeSwitchState::off());

        assert_eq!(on.to_string(), "state: on");
        assert_eq!(off.to_string(), "state: off");
        assert_eq!(
            serde_json::to_value(CliEvent::from(off)).unwrap(),
            json!({"event": "smart_home_switch", "state": false})
        );
    }

    #[test]
    fn renders_the_pairing_payload() {
        let event = SmartHomeStartPairingEvent::new(SmartHomePairingPayload {
            available_until: Some(1761582532251),
            qr_code: Some("MT:Y.K90".to_string()),
            manual_code: Some("34970112332".to_string()),
        });

        assert_eq!(
            event.to_string(),
            "available until: 1761582532251\nqr code: MT:Y.K90\nmanual code: 34970112332"
        );
    }
}
