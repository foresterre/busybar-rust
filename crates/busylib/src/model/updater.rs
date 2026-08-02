use serde::{Deserialize, Serialize};

use crate::types::TimeOfDay;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateStatus {
    pub install: Option<InstallStatus>,
    pub check: Option<CheckStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallStatus {
    pub is_allowed: Option<bool>,
    pub event: Option<UpdateEvent>,
    pub action: Option<UpdateAction>,
    pub status: Option<InstallResult>,
    pub detail: Option<String>,
    pub download: Option<DownloadProgress>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub speed_bytes_per_sec: Option<u64>,
    pub received_bytes: Option<u64>,
    pub total_bytes: Option<u64>,
}

impl DownloadProgress {
    pub fn fraction(&self) -> Option<f64> {
        match (self.received_bytes, self.total_bytes) {
            (Some(received), Some(total)) if total > 0 => {
                Some((received as f64 / total as f64).clamp(0.0, 1.0))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateEvent {
    SessionStart,
    SessionStop,
    ActionBegin,
    ActionDone,
    DetailChange,
    ActionProgress,
    None,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAction {
    Download,
    ShaVerification,
    Unpack,
    Prepare,
    Apply,
    None,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallResult {
    Ok,
    BatteryLow,
    Busy,
    DownloadFailure,
    DownloadAbort,
    ShaMismatch,
    UnpackStagingDirFailure,
    UnpackArchiveOpenFailure,
    UnpackArchiveUnpackFailure,
    InstallManifestNotFound,
    InstallManifestInvalid,
    InstallSessionConfigFailure,
    InstallPointerSetupFailure,
    UnknownFailure,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckStatus {
    pub available_version: Option<String>,
    pub event: Option<CheckEvent>,
    pub status: Option<CheckResult>,
}

impl CheckStatus {
    pub fn available_version(&self) -> Option<&str> {
        self.available_version
            .as_deref()
            .filter(|version| !version.is_empty())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckEvent {
    Start,
    Stop,
    None,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckResult {
    Available,
    NotAvailable,
    Failure,
    None,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoupdateSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_start: Option<TimeOfDay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_end: Option<TimeOfDay>,
}

impl AutoupdateSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.is_enabled = Some(enabled);
        self
    }

    pub fn window(mut self, start: TimeOfDay, end: TimeOfDay) -> Self {
        self.interval_start = Some(start);
        self.interval_end = Some(end);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct ChangelogResponse {
    #[serde(default)]
    pub changelog: String,
}
