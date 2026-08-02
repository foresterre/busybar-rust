use std::fmt;

use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::binary::Payload;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StreamingScreenEvent(Payload);

impl StreamingScreenEvent {
    pub fn new(payload: Payload) -> Self {
        Self(payload)
    }
}

impl From<StreamingScreenEvent> for CliEvent {
    fn from(event: StreamingScreenEvent) -> Self {
        CliEvent::StreamingScreen(event)
    }
}

impl fmt::Display for StreamingScreenEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn reports_a_frame_as_a_file_or_as_base64() {
        let written = StreamingScreenEvent::new(Payload::written(b"AAAA", "front.txt".to_string()));

        assert_eq!(written.to_string(), "wrote 4 bytes to front.txt");
        assert_eq!(
            serde_json::to_value(CliEvent::from(written)).unwrap(),
            json!({"event": "streaming_screen", "bytes": 4, "path": "front.txt"})
        );
    }
}
