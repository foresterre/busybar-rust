use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmartHomePairingInfo {
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
    pub value: PairingStatus,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SmartHomePairing {
    #[serde(default, with = "crate::serde_util::option_string_u64")]
    pub available_until: Option<u64>,
    pub qr_code: Option<String>,
    pub manual_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SmartHomeSwitch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup: Option<SwitchStartup>,
}

impl SmartHomeSwitch {
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
