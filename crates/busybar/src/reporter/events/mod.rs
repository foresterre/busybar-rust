mod system;
mod wifi;

use std::fmt;

use serde::Serialize;

pub use crate::reporter::events::system::{
    SystemStatusDeviceEvent, SystemStatusEvent, SystemStatusFirmwareEvent, SystemStatusPowerEvent,
    SystemStatusSystemEvent, SystemTransportEvent, SystemVersionEvent,
};
pub use crate::reporter::events::wifi::WifiStatusEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum CliEvent {
    SystemVersion(SystemVersionEvent),
    SystemTransport(SystemTransportEvent),
    SystemStatus(Box<SystemStatusEvent>),
    SystemStatusDevice(SystemStatusDeviceEvent),
    SystemStatusFirmware(SystemStatusFirmwareEvent),
    SystemStatusSystem(SystemStatusSystemEvent),
    SystemStatusPower(SystemStatusPowerEvent),
    WifiStatus(WifiStatusEvent),
}

impl fmt::Display for CliEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliEvent::SystemVersion(event) => event.fmt(f),
            CliEvent::SystemTransport(event) => event.fmt(f),
            CliEvent::SystemStatus(event) => event.fmt(f),
            CliEvent::SystemStatusDevice(event) => event.fmt(f),
            CliEvent::SystemStatusFirmware(event) => event.fmt(f),
            CliEvent::SystemStatusSystem(event) => event.fmt(f),
            CliEvent::SystemStatusPower(event) => event.fmt(f),
            CliEvent::WifiStatus(event) => event.fmt(f),
        }
    }
}
