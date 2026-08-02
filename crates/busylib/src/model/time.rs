use serde::{Deserialize, Serialize};

use crate::types::Timestamp;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimezoneInfo {
    /// Time zone name
    pub name: String,
    /// Time zone offset from UTC
    pub offset: String,
    /// Time zone abbreviation
    pub abbr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct TimestampInfo {
    /// ISO 8601 formatted timestamp with timezone
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct TimezoneListResponse {
    #[serde(default)]
    pub list: Vec<TimezoneInfo>,
}
