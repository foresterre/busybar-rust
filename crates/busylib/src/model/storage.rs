//! Storage schemas

use serde::{Deserialize, Serialize};

/// Element type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StorageListElement {
    File {
        /// File or directory name
        name: String,
        /// File size in bytes
        size: u64,
    },
    Dir {
        /// File or directory name
        name: String,
    },
}

impl StorageListElement {
    pub fn name(&self) -> &str {
        match self {
            StorageListElement::File { name, .. } | StorageListElement::Dir { name } => name,
        }
    }

    pub fn size(&self) -> Option<u64> {
        match self {
            StorageListElement::File { size, .. } => Some(*size),
            StorageListElement::Dir { .. } => None,
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, StorageListElement::Dir { .. })
    }
}

/// Usage of the internal storage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageStatus {
    pub used_bytes: Option<u64>,
    pub free_bytes: Option<u64>,
    /// Total size of the partition
    pub total_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct StorageList {
    /// Array of elements to display
    pub list: Vec<StorageListElement>,
}
