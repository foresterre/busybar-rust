use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiStatus {
    pub state: WifiState,
    pub ssid: Option<String>,
    pub bssid: Option<String>,
    pub channel: Option<u16>,
    pub rssi: Option<i32>,
    pub security: Option<WifiSecurity>,
    pub ip_config: Option<WifiIpConfig>,
}

impl WifiStatus {
    pub fn is_connected(&self) -> bool {
        matches!(self.state, WifiState::Connected)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WifiState {
    Unknown,
    Disconnected,
    Connected,
    Connecting,
    Disconnecting,
    Reconnecting,
    #[serde(untagged)]
    Unrecognized(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WifiSecurity {
    Open,
    #[serde(rename = "WPA")]
    Wpa,
    #[serde(rename = "WPA2")]
    Wpa2,
    #[serde(rename = "WEP")]
    Wep,
    #[serde(rename = "WPA/WPA2")]
    WpaWpa2,
    #[serde(rename = "WPA3")]
    Wpa3,
    #[serde(rename = "WPA2/WPA3")]
    Wpa2Wpa3,
    Unsupported,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiIpConfig {
    pub ip_method: Option<WifiIpMethod>,
    pub ip_type: Option<WifiIpType>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WifiIpMethod {
    Dhcp,
    Static,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WifiIpType {
    Ipv4,
    Ipv6,
}
