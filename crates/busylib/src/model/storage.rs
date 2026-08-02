use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StorageEntry {
    File { name: String, size: u64 },
    Dir { name: String },
}

impl StorageEntry {
    pub fn name(&self) -> &str {
        match self {
            StorageEntry::File { name, .. } | StorageEntry::Dir { name } => name,
        }
    }

    pub fn size(&self) -> Option<u64> {
        match self {
            StorageEntry::File { size, .. } => Some(*size),
            StorageEntry::Dir { .. } => None,
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, StorageEntry::Dir { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageStatus {
    pub used_bytes: Option<u64>,
    pub free_bytes: Option<u64>,
    pub total_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct StorageListResponse {
    pub list: Vec<StorageEntry>,
}
