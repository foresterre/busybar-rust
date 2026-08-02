use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub device: Option<DeviceInfo>,
    pub firmware: Option<FirmwareInfo>,
    pub system: Option<SystemInfo>,
    pub power: Option<PowerInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub serial_number: String,
    pub usb_mac: String,
    pub wifi_mac: Option<String>,
    pub ble_mac: Option<String>,
    pub otp_valid: bool,
    pub otp_model: Option<String>,
    pub otp_timestamp: Option<u64>,
    pub firmware_security: FirmwareSecurity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirmwareSecurity {
    Secure,
    Insecure,
    Other,
    Unknown,
    #[serde(untagged)]
    Unrecognized(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirmwareInfo {
    pub version: String,
    pub target: i64,
    pub branch: String,
    pub build_date: String,
    pub commit_hash: String,
    pub intercom_version: String,
    pub nwp_version: Option<String>,
    pub matter_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemInfo {
    pub api_semver: String,
    pub uptime: String,
    pub boot_time: i64,
    pub auto_update_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerInfo {
    pub state: PowerState,
    pub battery_charge: i32,
    pub battery_voltage: i32,
    pub battery_current: i32,
    pub usb_voltage: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PowerState {
    Discharging,
    Charging,
    Charged,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportType {
    Usb,
    Wifi,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct VersionInfo {
    pub api_semver: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct NetworkInterfaceInfo {
    pub r#type: TransportType,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct LogDumpResponse {
    pub path: String,
}
