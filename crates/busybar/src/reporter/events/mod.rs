mod wifi;

use std::fmt;

use serde::Serialize;

pub use crate::reporter::events::wifi::WifiStatusEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum CliEvent {
    WifiStatus(WifiStatusEvent),
}

impl fmt::Display for CliEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliEvent::WifiStatus(event) => event.fmt(f),
        }
    }
}
