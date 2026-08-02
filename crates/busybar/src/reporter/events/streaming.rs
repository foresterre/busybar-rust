use std::fmt;

use busylib::proto::bsb_error::{Cause, Severity};
use busylib::proto::bsb_state::State;
use busylib::proto::bsb_state::state_update::State as Update;
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::binary::Payload;
use crate::types::frame::{encoding_name, format_name, screen_name};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StreamingScreenEvent(Payload);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StreamError {
    cause: &'static str,
    severity: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Labelled {
    name: &'static str,
    value: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FramePayload {
    screen: Labelled,
    width: u32,
    height: u32,
    encoding: Labelled,
    pixel_format: Labelled,
    bytes: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

impl FramePayload {
    pub fn new(
        frame: &busylib::proto::bsb_frame::Frame,
        image: Option<(String, String)>,
        path: Option<String>,
        reason: Option<String>,
    ) -> Self {
        let (image_format, base64) = match image {
            Some((format, encoded)) => (Some(format), Some(encoded)),
            None => (None, None),
        };

        Self {
            screen: Labelled {
                name: screen_name(frame.screen()),
                value: frame.screen,
            },
            width: frame.width,
            height: frame.height,
            encoding: Labelled {
                name: encoding_name(frame.encoding()),
                value: frame.encoding,
            },
            pixel_format: Labelled {
                name: format_name(frame.pixel_format()),
                value: frame.pixel_format,
            },
            bytes: frame.data.len(),
            image_format,
            base64,
            path,
            reason,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StreamingStatusEvent {
    sequence: u64,
    timestamp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    frame: Option<FramePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<Update>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<StreamError>,
}

impl StreamingStatusEvent {
    pub fn error_of(state: &State) -> Option<StreamError> {
        state.error.as_ref().map(|error| StreamError {
            cause: cause_label(error.cause()),
            severity: severity_label(error.severity()),
        })
    }

    pub fn heartbeat(sequence: u64, timestamp: u64, error: Option<StreamError>) -> Self {
        Self {
            sequence,
            timestamp,
            frame: None,
            update: None,
            error,
        }
    }

    pub fn frame(
        sequence: u64,
        timestamp: u64,
        error: Option<StreamError>,
        frame: FramePayload,
    ) -> Self {
        Self {
            sequence,
            timestamp,
            frame: Some(frame),
            update: None,
            error,
        }
    }

    pub fn update(
        sequence: u64,
        timestamp: u64,
        error: Option<StreamError>,
        update: Update,
    ) -> Self {
        Self {
            sequence,
            timestamp,
            frame: None,
            update: Some(update),
            error,
        }
    }
}

impl From<StreamingStatusEvent> for CliEvent {
    fn from(event: StreamingStatusEvent) -> Self {
        CliEvent::StreamingStatus(Box::new(event))
    }
}

impl fmt::Display for StreamingStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{} ", self.sequence)?;

        match (&self.frame, &self.update) {
            (Some(frame), _) => write_frame(f, frame)?,
            (None, Some(update)) => write_update(f, update)?,
            (None, None) => f.write_str("heartbeat")?,
        }

        match &self.error {
            Some(StreamError { cause, severity }) => write!(f, " ({severity}: {cause})"),
            None => Ok(()),
        }
    }
}

fn write_frame(f: &mut fmt::Formatter<'_>, frame: &FramePayload) -> fmt::Result {
    write!(
        f,
        "frame {} {}x{} {} {}",
        frame.screen.name, frame.width, frame.height, frame.encoding.name, frame.pixel_format.name
    )?;

    match (&frame.path, &frame.reason) {
        (Some(path), _) => write!(f, " -> {path}"),
        (None, Some(reason)) => write!(f, " ({reason})"),
        (None, None) => Ok(()),
    }
}

fn write_update(f: &mut fmt::Formatter<'_>, update: &Update) -> fmt::Result {
    match update {
        Update::Brightness(brightness) => {
            write!(f, "brightness {}", brightness.actual_brightness)
        }
        Update::AudioVolume(volume) => write!(f, "audio volume {}", volume.volume),
        Update::DeviceName(name) => write!(f, "device name {}", name.name),
        other => write!(f, "{}", update_label(other)),
    }
}

fn update_label(update: &Update) -> &'static str {
    match update {
        Update::DeviceName(_) => "device name",
        Update::Power(_) => "power",
        Update::Brightness(_) => "brightness",
        Update::AudioVolume(_) => "audio volume",
        Update::Wifi(_) => "wifi",
        Update::UpdateState(_) => "update state",
        Update::UpdateCheck(_) => "update check",
        Update::Timezone(_) => "timezone",
        Update::Matter(_) => "matter",
        Update::Frame(_) => "frame",
        Update::Input(_) => "input",
        Update::Timer(_) => "timer",
        Update::Ble(_) => "ble",
        Update::AutoUpdateState(_) => "auto update state",
        Update::TimerProfiles(_) => "timer profiles",
    }
}

fn cause_label(cause: Cause) -> &'static str {
    match cause {
        Cause::ResourceLimit => "resource limit",
    }
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Fatal => "fatal",
        Severity::Error => "error",
        Severity::Warning => "warning",
    }
}

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
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;
    use busylib::proto::bsb_frame::{Encoding, Frame, PixelFormat, Screen};
    use busylib::proto::bsb_state::Brightness;
    use serde_json::json;

    use super::*;

    fn frame() -> Frame {
        Frame {
            screen: Screen::Front as i32,
            width: 72,
            height: 16,
            encoding: Encoding::RunLength as i32,
            pixel_format: PixelFormat::Rgb888 as i32,
            data: vec![0x7f, 0, 0, 0],
        }
    }

    #[test]
    fn numbers_each_message_it_reports() {
        let event = StreamingStatusEvent::heartbeat(7, 1785692728867, None);

        assert_eq!(event.to_string(), "#7 heartbeat");
        assert_eq!(
            serde_json::to_value(CliEvent::from(event)).unwrap(),
            json!({"event": "streaming_status", "sequence": 7, "timestamp": 1785692728867u64})
        );
    }

    #[test]
    fn reports_a_frame_with_both_label_and_wire_value() {
        let payload = FramePayload::new(
            &frame(),
            Some(("bmp".to_owned(), STANDARD.encode(b"image"))),
            None,
            None,
        );
        let event = StreamingStatusEvent::frame(1, 1785692728867, None, payload);

        assert_eq!(event.to_string(), "#1 frame front 72x16 run-length rgb888");

        let json = serde_json::to_value(CliEvent::from(event)).unwrap();
        let frame = &json["frame"];

        assert_eq!(frame["screen"], json!({"name": "front", "value": 0}));
        assert_eq!(frame["encoding"], json!({"name": "run-length", "value": 1}));
        assert_eq!(frame["pixel_format"], json!({"name": "rgb888", "value": 0}));
        assert_eq!(frame["bytes"], 4);
        assert_eq!(frame["image_format"], "bmp");
        assert_eq!(frame["base64"], "aW1hZ2U=");
        assert!(frame.get("path").is_none());
    }

    #[test]
    fn reports_where_a_frame_was_written() {
        let payload = FramePayload::new(
            &frame(),
            Some(("png".to_owned(), STANDARD.encode(b"image"))),
            Some("frames/front-000001.png".to_owned()),
            None,
        );
        let event = StreamingStatusEvent::frame(1, 1785692728867, None, payload);

        assert_eq!(
            event.to_string(),
            "#1 frame front 72x16 run-length rgb888 -> frames/front-000001.png"
        );
    }

    #[test]
    fn leaves_out_the_image_when_it_is_not_reported() {
        let payload = FramePayload::new(&frame(), None, None, None);
        let event = StreamingStatusEvent::frame(1, 1785692728867, None, payload);

        let json = serde_json::to_value(CliEvent::from(event)).unwrap();
        let frame = &json["frame"];

        assert!(frame.get("base64").is_none());
        assert!(frame.get("image_format").is_none());
        assert_eq!(frame["bytes"], 4);
    }

    #[test]
    fn falls_back_to_the_raw_payload_and_says_why() {
        let payload = FramePayload::new(
            &frame(),
            Some(("raw".to_owned(), STANDARD.encode(b"device"))),
            None,
            Some("the device streamed a deflate frame, which is not supported yet".to_owned()),
        );
        let event = StreamingStatusEvent::frame(2, 1785692728867, None, payload);

        assert_eq!(
            event.to_string(),
            "#2 frame front 72x16 run-length rgb888 \
             (the device streamed a deflate frame, which is not supported yet)"
        );

        let json = serde_json::to_value(CliEvent::from(event)).unwrap();

        assert_eq!(json["frame"]["image_format"], "raw");
        assert_eq!(json["frame"]["base64"], "ZGV2aWNl");
        assert!(
            json["frame"]["reason"]
                .as_str()
                .unwrap()
                .contains("deflate")
        );
        assert!(json["frame"].get("path").is_none());
    }

    #[test]
    fn reports_other_updates_by_their_value() {
        let update = Update::Brightness(Brightness {
            setting: None,
            actual_brightness: 30,
        });
        let event = StreamingStatusEvent::update(3, 1785692728867, None, update);

        assert_eq!(event.to_string(), "#3 brightness 30");
    }

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
