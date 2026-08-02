use serde::{Deserialize, Serialize};

use crate::types::Timestamp;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimezoneInfo {
    pub name: String,
    pub offset: String,
    pub abbr: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct TimestampInfo {
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct TimezoneListResponse {
    #[serde(default)]
    pub list: Vec<TimezoneInfo>,
}
