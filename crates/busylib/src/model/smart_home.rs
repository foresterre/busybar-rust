use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmartHomePairingInfo {
    /// Number of smart homes (Matter "fabrics") that this device is paired with ("commissioned
    /// into")
    pub fabric_count: Option<u32>,
    pub latest_pairing_status: Option<PairingStatusInfo>,
}

impl SmartHomePairingInfo {
    pub fn is_paired(&self) -> bool {
        self.fabric_count.is_some_and(|count| count > 0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairingStatusInfo {
    /// Latest state of smart home pairing (Matter "commissioning") process. Note:
    /// "never_started" only refers to the current power cycle of the device; this status is not
    /// recorded across reboots.
    pub value: PairingStatus,
    /// UTC Unix second timestamp of latest state update. Only present when a status update has
    /// occurred.
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PairingStatus {
    NeverStarted,
    Started,
    CompletedSuccessfully,
    Failed,
    #[serde(untagged)]
    Unknown(String),
}

/// Set of information for pairing with a Matter smart home
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmartHomePairingPayload {
    /// Pairing with ("commissioning into") a Matter smart home using the provided payload is
    /// possible before this UTC Unix millisecond timestamp. Note: it's a number in a string.
    #[serde(default, with = "crate::serde_util::option_string_u64")]
    pub available_until: Option<u64>,
    /// Payload of the QR code for pairing with ("commissioning into") a smart home
    pub qr_code: Option<String>,
    /// Manual code for pairing with ("commissioning into") a smart home
    pub manual_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SmartHomeSwitchState {
    /// State of emulated switch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<bool>,
    /// State of emulated switch on startup. Never sent by the server, but can be specified by
    /// the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup: Option<SwitchStartup>,
}

impl SmartHomeSwitchState {
    pub fn on() -> Self {
        Self {
            state: Some(true),
            startup: None,
        }
    }

    pub fn off() -> Self {
        Self {
            state: Some(false),
            startup: None,
        }
    }

    pub fn startup(mut self, startup: SwitchStartup) -> Self {
        self.startup = Some(startup);
        self
    }

    pub fn is_on(&self) -> bool {
        self.state.unwrap_or(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwitchStartup {
    Off,
    On,
    Toggle,
    Last,
}
