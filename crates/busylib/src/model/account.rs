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
    /// MQTT server url to connect to
    pub server_url: String,
    /// Client certificate type to use
    pub client_cert_type: ClientCertType,
    /// Whether to ignore the server certificate
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
pub(crate) struct AccountStatus {
    pub status: MqttStatus,
}
