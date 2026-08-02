use std::fmt;

use busylib::model::storage::{StorageListElement, StorageStatus};
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::binary::Payload;
use crate::reporter::events::fields::{field, write_fields};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StorageListEvent {
    list: Vec<StorageListElement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StorageStatusEvent(StorageStatus);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StorageReadEvent(Payload);

impl StorageListEvent {
    pub fn new(list: Vec<StorageListElement>) -> Self {
        Self { list }
    }
}

impl StorageStatusEvent {
    pub fn new(status: StorageStatus) -> Self {
        Self(status)
    }
}

impl StorageReadEvent {
    pub fn new(payload: Payload) -> Self {
        Self(payload)
    }
}

impl From<StorageListEvent> for CliEvent {
    fn from(event: StorageListEvent) -> Self {
        CliEvent::StorageList(event)
    }
}

impl From<StorageStatusEvent> for CliEvent {
    fn from(event: StorageStatusEvent) -> Self {
        CliEvent::StorageStatus(event)
    }
}

impl From<StorageReadEvent> for CliEvent {
    fn from(event: StorageReadEvent) -> Self {
        CliEvent::StorageRead(event)
    }
}

impl fmt::Display for StorageListEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, element) in self.list.iter().enumerate() {
            if index > 0 {
                f.write_str("\n")?;
            }

            match element.size() {
                Some(size) => write!(f, "file {} {size}", element.name())?,
                None => write!(f, "dir  {}", element.name())?,
            }
        }

        Ok(())
    }
}

impl fmt::Display for StorageStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = &self.0;
        let mut fields = Vec::new();

        if let Some(used_bytes) = status.used_bytes {
            fields.push(field("used bytes", used_bytes));
        }

        if let Some(free_bytes) = status.free_bytes {
            fields.push(field("free bytes", free_bytes));
        }

        if let Some(total_bytes) = status.total_bytes {
            fields.push(field("total bytes", total_bytes));
        }

        write_fields(f, &fields)
    }
}

impl fmt::Display for StorageReadEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn lists_files_with_their_size_and_directories_without() {
        let event = StorageListEvent::new(vec![
            StorageListElement::File {
                name: ".sys_update.txt".to_string(),
                size: 31,
            },
            StorageListElement::Dir {
                name: "apps_data".to_string(),
            },
        ]);

        assert_eq!(event.to_string(), "file .sys_update.txt 31\ndir  apps_data");
        assert_eq!(
            serde_json::to_value(CliEvent::from(event)).unwrap(),
            json!({
                "event": "storage_list",
                "list": [
                    {"type": "file", "name": ".sys_update.txt", "size": 31},
                    {"type": "dir", "name": "apps_data"}
                ]
            })
        );
    }

    #[test]
    fn renders_the_storage_usage() {
        let event = StorageStatusEvent::new(StorageStatus {
            used_bytes: Some(76087296),
            free_bytes: Some(7465435136),
            total_bytes: None,
        });

        assert_eq!(
            event.to_string(),
            "used bytes: 76087296\nfree bytes: 7465435136"
        );
    }

    #[test]
    fn reports_a_read_as_a_file_or_as_base64() {
        let written = StorageReadEvent::new(Payload::written(b"payload", "a.png".to_string()));
        let inline = StorageReadEvent::new(Payload::inline(b"payload"));

        assert_eq!(written.to_string(), "wrote 7 bytes to a.png");
        assert_eq!(inline.to_string(), "7 bytes");
        assert_eq!(
            serde_json::to_value(CliEvent::from(inline)).unwrap(),
            json!({"event": "storage_read", "bytes": 7, "base64": "cGF5bG9hZA=="})
        );
    }
}
