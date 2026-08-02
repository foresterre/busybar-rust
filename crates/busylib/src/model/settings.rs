use serde::{Deserialize, Serialize};

use crate::types::{AccessKey, DeviceName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpAccess {
    Disabled,
    Enabled,
    Key(AccessKey),
}

impl HttpAccess {
    pub fn mode(&self) -> AccessMode {
        match self {
            HttpAccess::Disabled => AccessMode::Disabled,
            HttpAccess::Enabled => AccessMode::Enabled,
            HttpAccess::Key(_) => AccessMode::Key,
        }
    }

    pub fn key(&self) -> Option<&AccessKey> {
        match self {
            HttpAccess::Key(key) => Some(key),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMode {
    Disabled,
    Enabled,
    Key,
}

impl AccessMode {
    pub fn as_str(self) -> &'static str {
        match self {
            AccessMode::Disabled => "disabled",
            AccessMode::Enabled => "enabled",
            AccessMode::Key => "key",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpAccessInfo {
    /// Access mode
    pub mode: AccessMode,
    /// Access key was set and is valid
    pub key_valid: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NameInfo {
    /// Device name (letters, digits, spaces and common punctuation; no backtick or tilde)
    pub name: DeviceName,
}
