//! Wi-Fi schemas

use serde::{Deserialize, Serialize};

/// Wi-Fi status. Only `state` is always present.
///
/// Fields `ssid`, `bssid`, `channel`, `rssi`, `security`, and `ip_config` are only included
/// when state is "connected".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusResponse {
    pub state: WifiState,
    /// Only present when connected
    pub ssid: Option<String>,
    /// Only present when connected
    pub bssid: Option<String>,
    /// Only present when connected
    pub channel: Option<u16>,
    /// Only present when connected
    pub rssi: Option<i32>,
    /// Only present when connected
    pub security: Option<WifiSecurityMethod>,
    /// Only present when connected
    pub ip_config: Option<WifiIpConfig>,
}

impl StatusResponse {
    pub fn is_connected(&self) -> bool {
        matches!(self.state, WifiState::Connected)
    }
}

/// States the Wi-Fi connection can be in
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

/// Security methods a network can use
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WifiSecurityMethod {
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

/// IP configuration of the connection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WifiIpConfig {
    pub ip_method: Option<WifiIpMethod>,
    pub ip_type: Option<WifiIpType>,
    pub address: Option<String>,
}

/// How an IP address is obtained
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WifiIpMethod {
    Dhcp,
    Static,
}

/// IP address families
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WifiIpType {
    Ipv4,
    Ipv6,
}
