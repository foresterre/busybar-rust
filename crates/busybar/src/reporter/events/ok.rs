use std::fmt;

use serde::Serialize;

use crate::reporter::events::CliEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OkEvent {
    operation: &'static str,
}

impl OkEvent {
    pub fn new(operation: &'static str) -> Self {
        Self { operation }
    }
}

impl From<OkEvent> for CliEvent {
    fn from(event: OkEvent) -> Self {
        CliEvent::Ok(event)
    }
}

impl fmt::Display for OkEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ok")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn reports_the_operation_that_succeeded() {
        let event = OkEvent::new("ble enable");

        assert_eq!(event.to_string(), "ok");
        assert_eq!(
            serde_json::to_value(CliEvent::from(event)).unwrap(),
            json!({"event": "ok", "operation": "ble enable"})
        );
    }
}
