use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountInfo {
    pub linked: Option<bool>,
    pub id: Option<String>,
    pub email: Option<String>,
    pub user_id: Option<String>,
}

impl AccountInfo {
    pub fn is_linked(&self) -> bool {
        self.linked.unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MqttStatus {
    Error,
    Disconnected,
    Connected,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountBackend {
    pub server_url: String,
    pub client_cert_type: ClientCertType,
    pub ignore_server_cert: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientCertType {
    Default,
    Custom,
    None,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct AccountStatusResponse {
    pub status: MqttStatus,
}
