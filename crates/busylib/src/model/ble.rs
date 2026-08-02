use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BleStatusResponse {
    pub status: BleState,
    /// Remote device address. Only present when status is "connected".
    pub address: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BleState {
    Reset,
    Initialization,
    Disabled,
    Enabled,
    Connectable,
    Connected,
    #[serde(rename = "internal error")]
    InternalError,
    #[serde(untagged)]
    Unknown(String),
}
