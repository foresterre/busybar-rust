use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status {
    pub device: Option<StatusDevice>,
    pub firmware: Option<StatusFirmware>,
    pub system: Option<StatusSystem>,
    pub power: Option<StatusPower>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusDevice {
    /// Device serial number
    pub serial_number: String,
    /// MAC of USB ethernet device
    pub usb_mac: String,
    /// WIFI MAC
    pub wifi_mac: Option<String>,
    /// BLE MAC
    pub ble_mac: Option<String>,
    /// Is OTP data valid
    pub otp_valid: bool,
    /// Device model code
    pub otp_model: Option<String>,
    /// Production timestamp
    pub otp_timestamp: Option<u64>,
    /// Summary of firmware signature protection derived from the wireless coprocessor (Si917)
    /// NWP and M4 signature state. "secure" - both NWP and M4 firmware signature verification
    /// active; "insecure" - neither active; "other" - mixed state (exactly one active);
    /// "unknown" - coprocessor info not ready yet.
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
pub struct StatusFirmware {
    /// Firmware version
    pub version: String,
    /// Firmware target code
    pub target: i64,
    /// Git branch name
    pub branch: String,
    /// Build date
    pub build_date: String,
    /// Git commit hash (may include -dirty suffix)
    pub commit_hash: String,
    /// Intercom handshake version string (forced version, git hash, or "intercom" if check
    /// disabled)
    pub intercom_version: String,
    /// Radio firmware version
    pub nwp_version: Option<String>,
    /// Matter version
    pub matter_version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusSystem {
    /// API SemVer
    pub api_semver: String,
    /// System uptime
    pub uptime: String,
    /// System boot timestamp
    pub boot_time: i64,
    /// Is auto-update enabled
    pub auto_update_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusPower {
    /// Power state
    pub state: PowerState,
    /// Battery charge percent
    pub battery_charge: i32,
    /// Battery voltage in mV
    pub battery_voltage: i32,
    /// Battery current in mA
    pub battery_current: i32,
    /// USB voltage in mV
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
    /// API SemVer
    pub api_semver: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct NetworkInterfaceInfo {
    /// Connection type
    pub r#type: TransportType,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct LogDumpResponse {
    pub path: String,
}
